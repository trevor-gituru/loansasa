use actix_web::{HttpResponse, Responder, HttpRequest, web};
use crate::db_operations::session::generate_session_id;
use crate::models::app_state::AppState;
use crate::models::session::Session;
use crate::utils::client_info::{get_browser, get_ip};
use deadpool_redis::redis::AsyncCommands;
use serde_json::{to_string, from_str};

pub async fn client(req: HttpRequest) -> impl Responder {
    let client_browser = get_browser(&req).unwrap_or_else(|| "unknown browser".to_string());
    let client_ip = get_ip(&req).unwrap_or_else(|| "unknown".to_string());
    HttpResponse::Ok().body(format!("Client IP: {}\nClient Browser: {}", client_ip, client_browser))
}

pub async fn test_redis(app_state: web::Data<AppState>, req: HttpRequest) -> impl Responder {
    let client_browser = get_browser(&req);
    let client_ip = get_ip(&req);
    // Get a connection from the pool
    let mut conn = app_state.redis_pool.get().await.expect("Failed to get redis connection");
    let session_id = generate_session_id(&mut conn);
    let client_session = Session::new(session_id.await, 5,client_ip, client_browser);

    // Use the connection to interact with Redis
    let _: () = conn.set_ex(
        format!("session: {}", client_session.session_id), 
        to_string(&client_session).unwrap(), 
        5)
        .await.
        unwrap();
    let result: String = conn.get(
        format!("session: {}", client_session.session_id)
    )
        .await
        .unwrap();
    let sess: Session = from_str(&result).unwrap();
    let exis: bool = conn.exists(format!("session: {}", client_session.session_id)).await.expect("Error check existance");

    HttpResponse::Ok().body(format!("Got value from Redis: {}\n\n{}", sess.session_id, exis))

        
}
