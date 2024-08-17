mod controllers;
mod db_operations;
mod models;
mod schema;
mod utils;

use std::sync::Arc;

use actix_web::{web, App, HttpServer};
use actix_files::Files;
use crate::controllers::auth::{register_get, login_get,register_post, login_post};
use crate::db_operations::connections::{establish_db_connection, establish_redis_connection};
use crate::models::app_state::AppState;
use crate::controllers::tests::client;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize the connection pool
    let db_pool = establish_db_connection();
    let redis_pool = establish_redis_connection();


    // Create app state with the connection pool
    let app_state = AppState { 
        db_pool: Arc::new(db_pool),
        redis_pool: Arc::new(redis_pool),
    };
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .service(Files::new("/assets", "./assets"))
            .service(
                web::scope("/auth")
                .route("/register", web::get().to(register_get))
                .route("/login", web::get().to(login_get))
                .route("/register", web::post().to(register_post))
                .route("/login", web::post().to(login_post))
            )
            .service(
                web::scope("/tests")
                .route("/client", web::get().to(client))
            )

            

    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
