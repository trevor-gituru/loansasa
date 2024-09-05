use actix_web::{HttpResponse, HttpRequest, Responder, web};
use askama::Template;
use std::sync::Arc;

use crate::db_operations::users::{create_user, verify_password, find_user};
use crate::db_operations::session::{check_session, create_session, delete_session};
use crate::db_operations::connections::RedisPool;
use crate::models::app_state::AppState;
use crate::models::ui::{RegisterTemplate, LoginTemplate};
use crate::models::users::{User, NewUser, RegisterForm, LoginForm};



// Get Controllers
pub async fn register_get() -> impl Responder {
    let template = RegisterTemplate{
        name: "",
        email: "",
        error : None,
    };
    HttpResponse::Ok().content_type("text/html").body(template.render().unwrap())
}
pub async fn login_get() -> impl Responder {
    let template = LoginTemplate{
        identifier: "",
        error : None,
    };
    HttpResponse::Ok().content_type("text/html").body(template.render().unwrap())
}
// Post Controllers
pub async fn register_post(
    reg_form: web::Form<RegisterForm>, 
    app_state: web::Data<AppState>,
    req: HttpRequest
) -> impl Responder {
    
    let new_user = NewUser {
        name: &reg_form.name,
        email: &reg_form.email,
        password: &reg_form.password,
    };
    println!("\n\nRegistration Data is:\n{:#?}", new_user);
    match app_state.db_pool.get() {
        Ok(mut conn) => {
            match create_user(&new_user, &mut conn) {
                Ok(user) => {
                    println!("Successfully created user:\n{user}");
                    
                    handle_login(user, app_state.redis_pool.clone(), req).await
                }
                Err(e) => {
                    let msg = "Username or email already exists:\n";
                    println!("{msg}{e:#?}");
                    register_error(&new_user, &msg,4 )
                    
                }
            }
        }
        Err(e) => {
            // Failed to get a connection
            // Handle the error here, e.g., log it, return an error response, etc.
            let msg = "Unable connect to database";
            println!("{msg}:\n{e:#?}");
            register_error(&new_user, &msg, 5)

        }
    }

}

pub async fn login_post(
    login_form: web::Form<LoginForm>, 
    app_state: web::Data<AppState>,
    req: HttpRequest
) -> impl Responder {
    println!("\n\nLog In Data is:\n{:#?}", login_form);
    match app_state.db_pool.get() {
        Ok(mut conn) => {
            match find_user(&login_form.identifier, &mut conn) {
                Ok(user) => {
                    match verify_password(&user, &login_form.password){
                        true => {
                            handle_login(user, app_state.redis_pool.clone(), req).await
                        }
                        false => {
                            let msg = "Username/Email and password do not match :\n";

                            login_error(&login_form.identifier, &msg,4 )
                        }
                    }
                }
                Err(e) => {
                    let msg = "Username/Email does not exist :\n";
                    println!("{msg}{e:#?}");
                    login_error(&login_form.identifier, &msg,4 )
                    
                }
            }
        }
        Err(e) => {
            // Failed to get a connection
            // Handle the error here, e.g., log it, return an error response, etc.
            let msg = "Unable connect to database";
            println!("{msg}:\n{e:#?}");
            login_error(&login_form.identifier, &msg, 5)

        }
    }
}
// Error Handlers
fn register_error(fail_user: &NewUser<'_>, err: &str, stats: u8) -> HttpResponse {
    let template = RegisterTemplate{
        name: &fail_user.name,
        email: &fail_user.email,
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
fn login_error(fail_identifier: &str, err: &str, stats: u8) -> HttpResponse {
    let template = LoginTemplate{
        identifier: &fail_identifier,
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
async fn handle_login(
    user: User,
    redis_pool: Arc<RedisPool>,
    req: HttpRequest) -> HttpResponse {
        let mut conn = redis_pool.get().await.expect("Error getting redis connection");
        
        let user_session = create_session(
            &mut conn, 
            req, 
            user.id).await;
        
        println!("{} successfully logged in\n", user.id);
        HttpResponse::Found()
            .insert_header(("LOCATION", "/dashboard/home"))
            .cookie(user_session)
            .finish()
}

pub async fn logout(
    app_state: web::Data<AppState>,
    req: HttpRequest) -> impl Responder {
        let err_msg = "Error getting redis connection in logout";
        let mut conn = app_state.redis_pool.get().await.expect(&err_msg);
        // Create a base response with a redirect
        if check_session(&mut conn, &req).await {
            let empty_cookie = delete_session(&mut conn, &req).await;
            HttpResponse::Found()
            .insert_header(("LOCATION", "/auth/login"))
            .cookie(empty_cookie)
            .finish()
        } else {
        HttpResponse::Found()
            .insert_header(("LOCATION", "/auth/login"))
            .finish()
        }
}



