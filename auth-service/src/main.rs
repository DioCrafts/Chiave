// src/main.rs
use actix_web::{App, HttpServer};
use crate::config::load_config;
use crate::handlers::{login, token, oauth};

mod config;
mod handlers;
mod jwt;
mod error;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = load_config();
    HttpServer::new(move || {
        App::new()
            .service(login::login_handler)
            .service(token::refresh_token)
            .service(oauth::oauth_login)
    })
    .bind(config.server_addr)?
    .run()
    .await
}

