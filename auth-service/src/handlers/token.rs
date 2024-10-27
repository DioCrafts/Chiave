// src/handlers/token.rs
use actix_web::{post, HttpResponse, Responder};

#[post("/refresh")]
async fn refresh_token_handler() -> impl Responder {
    // Lógica para verificar y renovar el token
    HttpResponse::Ok().json("Token refreshed")
}

