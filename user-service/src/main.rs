// src/main.rs
use actix_web::{App, HttpServer};
use crate::config::load_config;
use crate::handlers::{create_user, update_user, delete_user};

mod config;
mod handlers;
mod db;
mod error;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = load_config();
    let pool = db::establish_connection(&config.database_url);
    
    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .service(create_user::create_user)
            .service(update_user::update_user)
            .service(delete_user::delete_user)
    })
    .bind(config.server_addr)?
    .run()
    .await
}

