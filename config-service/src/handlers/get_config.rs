// src/handlers/get_config.rs
use actix_web::{get, web, HttpResponse, Responder};
use crate::config::Config;

#[get("/config")]
pub async fn get_config(config: web::Data<Config>) -> impl Responder {
    HttpResponse::Ok().json(config.get_ref().clone())
}

