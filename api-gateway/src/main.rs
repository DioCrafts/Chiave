// src/main.rs
use actix_web::{App, HttpServer};
use crate::config::load_config;
use crate::routes::{auth, users, roles};
use crate::middleware::auth_middleware;

mod config;
mod routes;
mod middleware;
mod error;
mod models;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = load_config();
    HttpServer::new(move || {
        App::new()
            .wrap(auth_middleware::AuthMiddleware)
            .configure(auth::config_routes)
            .configure(users::config_routes)
            .configure(roles::config_routes)
    })
    .bind(config.server_addr)?
    .run()
    .await
}

