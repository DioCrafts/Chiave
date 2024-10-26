// src/db/models.rs
use serde::{Deserialize, Serialize};
use diesel::prelude::*;

#[derive(Queryable, Serialize)]
pub struct Role {
    pub id: i32,
    pub name: String,
}

#[derive(Insertable, Deserialize)]
#[table_name = "roles"]
pub struct NewRole {
    pub name: String,
}

#[derive(Queryable, Serialize)]
pub struct Policy {
    pub id: i32,
    pub role_id: i32,
    pub resource: String,
    pub action: String,
}

#[derive(Insertable, Deserialize)]
#[table_name = "policies"]
pub struct NewPolicy {
    pub role_id: i32,
    pub resource: String,
    pub action: String,
}

