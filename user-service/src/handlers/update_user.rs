// src/handlers/update_user.rs
use actix_web::{put, web, HttpResponse, Responder};
use crate::db::models::UpdateUser;
use crate::db::update_user_in_db;

#[put("/users/{id}")]
async fn update_user(user_id: web::Path<i32>, updated_user: web::Json<UpdateUser>, pool: web::Data<DbPool>) -> impl Responder {
    match update_user_in_db(user_id.into_inner(), updated_user.into_inner(), &pool).await {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(_) => HttpResponse::InternalServerError().json("Error updating user"),
    }
}

