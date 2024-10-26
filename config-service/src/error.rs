// src/error.rs
use actix_web::{HttpResponse, ResponseError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigServiceError {
    #[error("Configuration not found")]
    NotFound,
    #[error("Failed to update configuration")]
    UpdateFailed,
}

impl ResponseError for ConfigServiceError {
    fn error_response(&self) -> HttpResponse {
        match *self {
            ConfigServiceError::NotFound => HttpResponse::NotFound().json("Configuration not found"),
            ConfigServiceError::UpdateFailed => HttpResponse::InternalServerError().json("Failed to update configuration"),
        }
    }
}

