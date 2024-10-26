// unit_tests/test_db.rs
use user_service::db::{establish_connection, models::NewUser};

#[test]
fn test_db_connection() {
    let db_url = "test_database_url";
    let conn = establish_connection(db_url);
    assert!(conn.is_ok());
}

#[test]
fn test_insert_user() {
    let db_url = "test_database_url";
    let conn = establish_connection(db_url).unwrap();
    let new_user = NewUser {
        username: "test_user".to_string(),
        email: "test_user@example.com".to_string(),
        password: "password".to_string(),
    };
    
    let result = conn.execute("INSERT INTO users (username, email, password) VALUES ($1, $2, $3)",
                              &[&new_user.username, &new_user.email, &new_user.password]);
    assert!(result.is_ok());
}

