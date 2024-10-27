// src/handlers/login.rs
use actix_web::{post, web, HttpResponse, Responder};
use serde::Deserialize;
use crate::jwt::create_token;
use crate::config::Config;

#[derive(Deserialize)]
pub struct LoginRequest {
    username: String,
    password: String,
}

#[post("/login")]
async fn login_handler(credentials: web::Json<LoginRequest>, config: web::Data<Config>) -> impl Responder {
    // Verificación de credenciales (simplificada)
    if credentials.username == "user" && credentials.password == "password" {
        let token = create_token(&credentials.username, &config.jwt_secret);
        HttpResponse::Ok().json(token)
    } else {
        HttpResponse::Unauthorized().json("Invalid credentials")
    }
}

