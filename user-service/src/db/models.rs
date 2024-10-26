// src/db/models.rs
use serde::{Deserialize, Serialize};
use diesel::prelude::*;

#[derive(Queryable, Serialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
}

#[derive(Insertable, Deserialize)]
#[table_name = "users"]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(AsChangeset, Deserialize)]
#[table_name = "users"]
pub struct UpdateUser {
    pub email: Option<String>,
    pub password: Option<String>,
}

