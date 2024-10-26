// src/config.rs
use serde::Deserialize;
use std::env;

#[derive(Deserialize)]
pub struct Config {
    pub server_addr: String,
    pub database_url: String,
}

pub fn load_config() -> Config {
    dotenv::dotenv().ok();
    Config {
        server_addr: env::var("SERVER_ADDR").unwrap_or_else(|_| "127.0.0.1:8082".into()),
        database_url: env::var("DATABASE_URL").expect("Database URL not set"),
    }
}

