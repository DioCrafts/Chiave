// src/main.rs
use actix_web::{App, HttpServer};
use crate::config::load_config;
use crate::handlers::{get_config, set_config};

mod config;
mod handlers;
mod error;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = load_config();

    HttpServer::new(move || {
        App::new()
            .data(config.clone())
            .service(get_config::get_config)
            .service(set_config::set_config)
    })
    .bind(config.server_addr)?
    .run()
    .await
}

