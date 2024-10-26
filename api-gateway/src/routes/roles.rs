// src/routes/roles.rs
use actix_web::{get, post, web, HttpResponse, Responder};

#[get("/roles")]
async fn list_roles() -> impl Responder {
    // Lógica para listar roles
    HttpResponse::Ok().json("List of roles")
}

#[post("/roles")]
async fn create_role(new_role: web::Json<NewRole>) -> impl Responder {
    // Lógica para crear un rol
    HttpResponse::Created().json("Role created")
}

pub fn config_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(list_roles)
       .service(create_role);
}

