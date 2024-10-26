// src/handlers/delete_user.rs
use actix_web::{delete, web, HttpResponse, Responder};
use crate::db::delete_user_from_db;

#[delete("/users/{id}")]
async fn delete_user(user_id: web::Path<i32>, pool: web::Data<DbPool>) -> impl Responder {
    match delete_user_from_db(user_id.into_inner(), &pool).await {
        Ok(_) => HttpResponse::Ok().json("User deleted"),
        Err(_) => HttpResponse::InternalServerError().json("Error deleting user"),
    }
}

