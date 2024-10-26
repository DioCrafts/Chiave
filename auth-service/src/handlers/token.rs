// src/handlers/token.rs
use actix_web::{post, HttpResponse, Responder};
use crate::jwt::refresh_token;

#[post("/refresh")]
async fn refresh_token() -> impl Responder {
    // Lógica para verificar y renovar el token
    HttpResponse::Ok().json("Token refreshed")
}

