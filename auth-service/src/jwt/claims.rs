// src/jwt/claims.rs
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // Identificación del usuario
    pub exp: usize,  // Tiempo de expiración del token
}

