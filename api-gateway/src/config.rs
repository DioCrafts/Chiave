// src/config.rs
use serde::Deserialize;
use std::env;

#[derive(Deserialize)]
pub struct Config {
    pub server_addr: String,
    pub auth_service_url: String,
    pub user_service_url: String,
    pub role_service_url: String,
}

pub fn load_config() -> Config {
    dotenv::dotenv().ok();
    Config {
        server_addr: env::var("SERVER_ADDR").unwrap_or_else(|_| "127.0.0.1:8080".into()),
        auth_service_url: env::var("AUTH_SERVICE_URL").expect("Auth service URL not set"),
        user_service_url: env::var("USER_SERVICE_URL").expect("User service URL not set"),
        role_service_url: env::var("ROLE_SERVICE_URL").expect("Role service URL not set"),
    }
}

