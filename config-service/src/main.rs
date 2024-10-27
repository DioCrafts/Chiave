// src/main.rs
use actix_web::{App, HttpServer, web};
use crate::config::load_config;
use crate::handlers::{get_config, set_config};
use std::sync::Mutex;

mod config;
mod handlers;
mod error;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = load_config();
    let server_address = config.server_addr.clone(); // Clona la dirección antes de mover `config`

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(Mutex::new(config.clone())))
            .service(get_config::get_config)
            .service(set_config::set_config)
    })
    .bind(server_address)? // Usa la dirección clonada aquí
    .run()
    .await
}

