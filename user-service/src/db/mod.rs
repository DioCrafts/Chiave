// src/db/mod.rs
use diesel::r2d2::{self, ConnectionManager};
use diesel::PgConnection;
use std::env;

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn establish_connection(database_url: &str) -> DbPool {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}

// Funciones de gestión de usuarios
pub mod models;
pub mod operations;

