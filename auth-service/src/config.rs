// src/config.rs
use serde::Deserialize;
use std::env;

#[derive(Deserialize)]
pub struct Config {
    pub server_addr: String,
    pub jwt_secret: String,
    pub oauth_client_id: String,
    pub oauth_client_secret: String,
    pub oauth_redirect_url: String,
}

pub fn load_config() -> Config {
    dotenv::dotenv().ok();
    Config {
        server_addr: env::var("SERVER_ADDR").unwrap_or_else(|_| "127.0.0.1:8081".into()),
        jwt_secret: env::var("JWT_SECRET").expect("JWT secret not set"),
        oauth_client_id: env::var("OAUTH_CLIENT_ID").expect("OAuth client ID not set"),
        oauth_client_secret: env::var("OAUTH_CLIENT_SECRET").expect("OAuth client secret not set"),
        oauth_redirect_url: env::var("OAUTH_REDIRECT_URL").expect("OAuth redirect URL not set"),
    }
}

