// src/main.rs
use actix_web::{App, HttpServer};
use crate::config::load_config;
use crate::routes::{auth, users, roles};
use crate::middleware::auth_middleware;

mod config;
mod routes;
mod middleware;
mod error;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = load_config();
    HttpServer::new(move || {
        App::new()
            .wrap(auth_middleware::AuthMiddleware::new()) // Middleware de autenticación
            .service(auth::config_routes())
            .service(users::config_routes())
            .service(roles::config_routes())
    })
    .bind(config.server_addr)?
    .run()
    .await
}

