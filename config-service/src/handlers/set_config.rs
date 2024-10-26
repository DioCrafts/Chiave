// src/handlers/set_config.rs
use actix_web::{put, web, HttpResponse, Responder};
use crate::config::Config;
use std::sync::Mutex;

#[derive(Deserialize)]
pub struct UpdateConfigRequest {
    some_global_setting: Option<String>,
}

#[put("/config")]
async fn set_config(
    config: web::Data<Mutex<Config>>,
    new_config: web::Json<UpdateConfigRequest>,
) -> impl Responder {
    let mut config = config.lock().unwrap();

    if let Some(setting) = &new_config.some_global_setting {
        config.some_global_setting = setting.clone();
    }

    HttpResponse::Ok().json("Configuration updated")
}

