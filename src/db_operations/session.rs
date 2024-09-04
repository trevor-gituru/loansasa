use crate::models::session::Session;
use crate::utils::client_info::{get_browser, get_ip};
use actix_web::{cookie, HttpRequest};
use deadpool_redis::Connection;
use deadpool_redis::redis::AsyncCommands;
use rand::{Rng, thread_rng};
use rand::distributions::Alphanumeric;
use serde_json::to_string;

pub async fn generate_session_id(conn : &mut Connection) -> String {   
    // Creates a random number generator that is local to the current thread.
    //let mut rng = thread_rng();
    loop{
        
        let id = thread_rng()
            //  This is a distribution that generates random alphanumeric characters (a-z, A-Z, 0-9)
            .sample_iter(&Alphanumeric)
            // Generate 32 characters
            .take(32) 
            // Converts each generated u8 into a char
            .map(char::from)
            // Collects the characters into a String
            .collect();
        let sess_id = format!("Session:{}", id);
        let err_msg = "Error check existance of session id in generate session id";
        let exists: bool = conn.exists(&sess_id).await.expect(err_msg);
        if !exists{
            return id;
        }
    }

}

pub async fn create_session (
    conn: &mut Connection,
    req: HttpRequest,
    user_id: i32) -> cookie::Cookie<'_>{
        let user_browser = get_browser(&req);
        let user_ip = get_ip(&req);
        let session_id = generate_session_id(conn).await;
        let session = Session::new(
            session_id, 
            user_id, 
            user_ip,
            user_browser);
        let key = format!("Session:{}", &session.session_id);
        let value = to_string(&session).expect("Error serializing Session");
        let err_mssg = "Unable to create session";
        let _: () = conn.set_ex(key, value, 1800).await.expect(&err_mssg);
        let cookie = cookie::Cookie::build("session_id",
         session.session_id)
            .path("/")
            .http_only(true)
            .same_site(cookie::SameSite::Strict)
            .max_age(cookie::time::Duration::seconds(1800)) // Set the cookie to expire in 30 minutes
            .finish();
        cookie
}

pub async fn check_session (
    conn: &mut Connection,
    req: &HttpRequest) -> bool{
        // Get cookies from the request
        let session_cookies = req.cookie("session_id");
        if session_cookies.is_none(){
            return false;
        }
        let session_cookies = session_cookies.unwrap();
        let session_id = session_cookies.value();
        let key = format!("Session:{}", &session_id);
        println!("{}", key);
        let err_msg = "Error check existance of session id in check_session";
        let exists: bool = conn.exists(&key).await.expect(err_msg);
        exists
}

pub async fn delete_session (
    conn: &mut Connection,
    req: &HttpRequest) -> cookie::Cookie<'static>{
        // Get cookies from the request
        let session_cookies = req.cookie("session_id").unwrap();
        let session_id = session_cookies.value();
        let key = format!("Session:{}", &session_id);
        let err_msg = "Error deleting session id in delete_session";
        // Remove the session from Redis
        let _: () = conn.del(&key).await.expect(&err_msg);
        println!("{} was deleted", key);
        // Create a cookie with the same name and an expired date to delete it
        let cookie = cookie::Cookie::build("session_id", "")
            .path("/")
            .max_age(cookie::time::Duration::ZERO)
            .finish();
        cookie
}


