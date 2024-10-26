// src/handlers/create_user.rs
use actix_web::{post, web, HttpResponse, Responder};
use crate::db::models::NewUser;
use crate::db::create_user_in_db;

#[post("/users")]
async fn create_user(new_user: web::Json<NewUser>, pool: web::Data<DbPool>) -> impl Responder {
    match create_user_in_db(new_user.into_inner(), &pool).await {
        Ok(user) => HttpResponse::Created().json(user),
        Err(_) => HttpResponse::InternalServerError().json("Error creating user"),
    }
}

