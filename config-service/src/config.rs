// src/config.rs
use serde::{Deserialize, Serialize};
use std::env;
use std::sync::Mutex;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Config {
    pub server_addr: String,
    pub some_global_setting: String,
}

pub fn load_config() -> Config {
    dotenv::dotenv().ok();
    Config {
        server_addr: env::var("SERVER_ADDR").unwrap_or_else(|_| "127.0.0.1:8084".into()),
        some_global_setting: env::var("SOME_GLOBAL_SETTING").unwrap_or_else(|_| "default_value".into()),
    }
}

