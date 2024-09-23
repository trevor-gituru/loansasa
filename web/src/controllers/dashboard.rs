use actix_web::{HttpResponse, HttpRequest, web};
use diesel::PgConnection;

use askama::Template;
use crate::models::ui::{HomeTemplate, LoansTemplate, ProfileTemplate, LendersTemplate};
use crate::models::user_details::*;
use crate::models::app_state::AppState;
use crate::db_operations::session::{check_session, get_session};
use crate::db_operations::user_details::{create_user_details, find_user_details, update_user_details};

// Get Controllers
pub async fn dashboard_get(
  app_state: web::Data<AppState>,
  req: HttpRequest,
  path: web::Path<String>) -> HttpResponse {
    let mut conn = app_state.redis_pool.get().await.unwrap();
    if check_session(&mut conn, &req).await {
        let ses_id = get_session(&mut conn, &req).await;
        println!("Session exists {}\n", ses_id);
        let page = path.into_inner();
        let mut db_conn = app_state.db_pool.get().unwrap();
        let template = match page.as_str() {
            "profile" => profile_get(&mut db_conn, ses_id).await,
            "loans" => LoansTemplate{}.render(),
            "lenders" => LendersTemplate{}.render(),
            _ => HomeTemplate{}.render(),
        };
        HttpResponse::Ok().content_type("text/html").body(template.unwrap())
    }else{
        println!("Session doesnt exists\n");
        HttpResponse::Found()
        .insert_header(("LOCATION", "/auth/login"))
        .finish()
    }
    
}

// Post Controllers
pub async fn profile_post(
  app_state: web::Data<AppState>,
  profile_form: web::Form<ProfileForm>,
  req: HttpRequest) -> HttpResponse {
    let mut conn = app_state.redis_pool.get().await.unwrap();
    if check_session(&mut conn, &req).await {
        let ses_id = get_session(&mut conn, &req).await;
        println!("Session exists {}\n", ses_id);
        let new_user_details = NewUserDetails {
            user_id: ses_id,
            account_address: &profile_form.account_address,
            private_key: &profile_form.private_key,
        };
        println!("\n\nRegistration User Details Data is:\n{:#?}", new_user_details);
        match app_state.db_pool.get() {
            Ok(mut conn) => {
                let user_details = find_user_details(ses_id, &mut conn);
                if user_details.is_err(){
                    match create_user_details(&new_user_details, &mut conn) {
                        Ok(user_detail) => {
                            println!("Successfully created user details:\n{user_detail}");
                            
                            let template = profile_get(&mut conn, ses_id).await;
                            HttpResponse::Ok().content_type("text/html").body(template.unwrap())
        
                        }
                        Err(e) => {
                            let msg = "Failure create User Details exists:\n";
                            println!("{msg}{e:#?}");
                            profile_post_error(&new_user_details, &msg,4 )
                            
                        }
                    }
                
                } else {
                    match update_user_details(&new_user_details, &mut conn) {
                        Ok(user_detail) => {
                            println!("Successfully updated user details:\n{user_detail}");
                            
                            let template = profile_get(&mut conn, ses_id).await;
                            HttpResponse::Ok().content_type("text/html").body(template.unwrap())
        
                        }
                        Err(e) => {
                            let msg = "Failure create User Details exists:\n";
                            println!("{msg}{e:#?}");
                            profile_post_error(&new_user_details, &msg,4 )
                            
                        }
                    }
                
                }
               
            }
            Err(e) => {
                // Failed to get a connection
                // Handle the error here, e.g., log it, return an error response, etc.
                let msg = "Unable connect to database";
                println!("{msg}:\n{e:#?}");
                profile_post_error(&new_user_details, &msg, 5)
    
            }
        }
        
    }else{
        println!("Session doesnt exists\n");
        HttpResponse::Found()
        .insert_header(("LOCATION", "/auth/login"))
        .finish()
    }
    

}

fn profile_post_error(fail_user: &NewUserDetails<'_>, err: &str, stats: u8) -> HttpResponse {
    let template = ProfileTemplate{
        account_address: &fail_user.account_address,
        private_key: &fail_user.private_key,
        error: Some(err),
    };
    match stats {
        5 => {
            HttpResponse::InternalServerError().content_type("text/html").body(template.render().unwrap())
        }
        4 => {
            HttpResponse::BadRequest().content_type("text/html").body(template.render().unwrap())
            
        }
        _ => {
            HttpResponse::InternalServerError().content_type("text/html").body(template.render().unwrap())
        }
    }
}
async fn profile_get(conn: &mut PgConnection, sess_id: i32) -> Result<String, askama::Error> {
    let user_dets = find_user_details(sess_id, conn);
    if user_dets.is_err(){
        println!("No User Details Found");
        let template = ProfileTemplate{
            account_address: "",
            private_key: "",
            error : None,
        };
        template.render()
    } else {
        let user_dets = user_dets.unwrap();
        println!("User Details Found {:#?}", user_dets);
        let template = ProfileTemplate{
            account_address: &user_dets.account_address,
            private_key: &user_dets.account_address,
            error : None,
        };
        template.render()
    }
    
}