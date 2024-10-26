// src/error.rs
use actix_web::{HttpResponse, ResponseError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RoleServiceError {
    #[error("Role not found")]
    NotFound,
    #[error("Database error")]
    DatabaseError,
}

impl ResponseError for RoleServiceError {
    fn error_response(&self) -> HttpResponse {
        match *self {
            RoleServiceError::NotFound => HttpResponse::NotFound().json("Role not found"),
            RoleServiceError::DatabaseError => HttpResponse::InternalServerError().json("Database error"),
        }
    }
}

