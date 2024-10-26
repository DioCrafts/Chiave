// src/main.rs
use actix_web::{App, HttpServer};
use crate::config::load_config;
use crate::handlers::{create_role, assign_role, policy};

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
            .service(create_role::create_role)
            .service(assign_role::assign_role)
            .service(policy::define_policy)
    })
    .bind(config.server_addr)?
    .run()
    .await
}

