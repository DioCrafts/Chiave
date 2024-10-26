// src/handlers/assign_role.rs
use actix_web::{post, web, HttpResponse, Responder};
use crate::db::assign_role_to_user;

#[derive(Deserialize)]
pub struct RoleAssignment {
    user_id: i32,
    role_id: i32,
}

#[post("/roles/assign")]
async fn assign_role(assignment: web::Json<RoleAssignment>, pool: web::Data<DbPool>) -> impl Responder {
    match assign_role_to_user(assignment.user_id, assignment.role_id, &pool).await {
        Ok(_) => HttpResponse::Ok().json("Role assigned"),
        Err(_) => HttpResponse::InternalServerError().json("Error assigning role"),
    }
}

