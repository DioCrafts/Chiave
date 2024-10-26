// src/error.rs
use actix_web::{HttpResponse, ResponseError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AuthError {
    #[error("Unauthorized access")]
    Unauthorized,
    #[error("Token expired or invalid")]
    TokenExpired,
}

impl ResponseError for AuthError {
    fn error_response(&self) -> HttpResponse {
        match *self {
            AuthError::Unauthorized => HttpResponse::Unauthorized().json("Unauthorized access"),
            AuthError::TokenExpired => HttpResponse::Unauthorized().json("Token expired or invalid"),
        }
    }
}

