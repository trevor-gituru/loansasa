use actix_web::{HttpResponse, Responder};
use crate::models::ui::{RegisterTemplate};
use askama::Template;

pub async fn register_get() -> impl Responder {
    let template = RegisterTemplate{};
    HttpResponse::Ok().content_type("text/html").body(template.render().unwrap())
}
pub async fn register_post(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}
