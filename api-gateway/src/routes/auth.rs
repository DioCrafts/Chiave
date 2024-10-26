// src/routes/auth.rs
use actix_web::{post, web, HttpResponse, Responder};

#[post("/login")]
async fn login(credentials: web::Json<Credentials>) -> impl Responder {
    // Lógica para iniciar sesión y retornar un token
    HttpResponse::Ok().json("Token generated")
}

#[post("/refresh")]
async fn refresh_token() -> impl Responder {
    // Lógica para renovar el token
    HttpResponse::Ok().json("Token refreshed")
}

pub fn config_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(login)
       .service(refresh_token);
}

