use actix_web::{HttpResponse, Responder, HttpRequest};
use crate::utils::client_info::{get_browser, get_ip};

pub async fn client(req: HttpRequest) -> impl Responder {
    let client_browser = get_browser(&req).unwrap_or_else(|| "unknown browser".to_string());
    let client_ip = get_ip(&req).unwrap_or_else(|| "unknown".to_string());
    HttpResponse::Ok().body(format!("Client IP: {}\nClient Browser: {}", client_ip, client_browser))
}
