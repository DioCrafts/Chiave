// src/error.rs
use actix_web::{HttpResponse, ResponseError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum UserServiceError {
    #[error("User not found")]
    NotFound,
    #[error("Database error")]
    DatabaseError,
}

impl ResponseError for UserServiceError {
    fn error_response(&self) -> HttpResponse {
        match *self {
            UserServiceError::NotFound => HttpResponse::NotFound().json("User not found"),
            UserServiceError::DatabaseError => HttpResponse::InternalServerError().json("Database error"),
        }
    }
}

