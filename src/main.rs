mod controllers;
mod db_operations;
mod models;
mod schema;

use actix_web::{web, App, HttpServer};
use actix_files::Files;
use crate::controllers::auth::{register_get, register_post};
use crate::db_operations::db::establish_connection;
use crate::models::app_state::AppState;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize the connection pool
    let pool = establish_connection();

    // Create app state with the connection pool
    let app_state = AppState { pool };
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .service(Files::new("/assets", "./assets"))
            .route("/register", web::get().to(register_get))
            .route("/register", web::post().to(register_post))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
