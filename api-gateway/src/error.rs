// src/error.rs
use actix_web::{HttpResponse, ResponseError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Unauthorized access")]
    Unauthorized,
    #[error("Resource not found")]
    NotFound,
}

impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        match *self {
            ApiError::Unauthorized => HttpResponse::Unauthorized().json("Unauthorized access"),
            ApiError::NotFound => HttpResponse::NotFound().json("Resource not found"),
        }
    }
}

