// src/handlers/create_role.rs
use actix_web::{post, web, HttpResponse, Responder};
use crate::db::models::NewRole;
use crate::db::create_role_in_db;

#[post("/roles")]
async fn create_role(new_role: web::Json<NewRole>, pool: web::Data<DbPool>) -> impl Responder {
    match create_role_in_db(new_role.into_inner(), &pool).await {
        Ok(role) => HttpResponse::Created().json(role),
        Err(_) => HttpResponse::InternalServerError().json("Error creating role"),
    }
}

