mod controllers;
mod models;

use actix_web::{web, App, HttpServer};
use crate::controllers::auth::{register_get, register_post};
use actix_files::Files;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(Files::new("/assets", "./assets"))
            .route("/register", web::get().to(register_get))
            .route("/register", web::post().to(register_post))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}