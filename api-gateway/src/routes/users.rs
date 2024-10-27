// src/routes/users.rs
use actix_web::{get, post, put, delete, web, HttpResponse, Responder};
use crate::models::{NewUser, UpdateUser};

#[get("/users")]
async fn list_users() -> impl Responder {
    HttpResponse::Ok().json("List of users")
}

#[post("/users")]
async fn create_user(new_user: web::Json<NewUser>) -> impl Responder {
    HttpResponse::Created().json("User created")
}

#[put("/users/{id}")]
async fn update_user(user_id: web::Path<String>, updated_user: web::Json<UpdateUser>) -> impl Responder {
    HttpResponse::Ok().json("User updated")
}

#[delete("/users/{id}")]
async fn delete_user(user_id: web::Path<String>) -> impl Responder {
    HttpResponse::Ok().json("User deleted")
}

pub fn config_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(list_users)
       .service(create_user)
       .service(update_user)
       .service(delete_user);
}

