use actix_web::{HttpResponse, Responder, web};
use askama::Template;
use crate::db_operations::users::create_user;
use crate::models::app_state::AppState;
use crate::models::ui::{RegisterTemplate, LoginTemplate};
use crate::models::users::{NewUser, RegisterForm, User};
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
    app_state: web::Data<AppState>
) -> impl Responder {
    
    let new_user = NewUser {
        name: &reg_form.name,
        email: &reg_form.email,
        password: &reg_form.password,
    };
    println!("\n\nRegistration Data is:\n{:#?}", new_user);
    match app_state.pool.get() {
        Ok(mut conn) => {
            match create_user(&new_user, &mut conn) {
                Ok(user) => {
                    println!("Successfully created user:\n{user:#?}");
                    handle_login(user)
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

fn handle_login(user: User) -> HttpResponse {
    HttpResponse::Ok().body(format!("Successfully created\nid: {}\nname:\
    {}\nemail: {}\npassword: {}\ncreated_at: {}\n", user.id, user.name, user.email, 
    user.password, user.created_at))
}


