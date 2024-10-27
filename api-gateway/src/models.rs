// src/models.rs

use serde::Deserialize;

#[derive(Deserialize)]
pub struct Credentials {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct NewRole {
    pub name: String,
}

#[derive(Deserialize)]
pub struct NewUser {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct UpdateUser {
    pub username: Option<String>,
    pub password: Option<String>,
}

