// src/handlers/policy.rs
use actix_web::{post, web, HttpResponse, Responder};
use crate::db::models::Policy;
use crate::db::define_policy_in_db;

#[post("/policies")]
async fn define_policy(policy: web::Json<Policy>, pool: web::Data<DbPool>) -> impl Responder {
    match define_policy_in_db(policy.into_inner(), &pool).await {
        Ok(policy) => HttpResponse::Created().json(policy),
        Err(_) => HttpResponse::InternalServerError().json("Error defining policy"),
    }
}

