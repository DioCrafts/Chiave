// integration_tests/test_user.rs
use actix_web::test;
use user_service::{handlers::create_user, db::models::NewUser, config::Config};

#[actix_web::test]
async fn test_create_user() {
    let config = Config { database_url: "test_database_url".to_string(), ..Default::default() };
    let new_user = NewUser {
        username: "new_user".to_string(),
        email: "new_user@example.com".to_string(),
        password: "secure_password".to_string(),
    };
    
    let req = test::TestRequest::post()
        .uri("/users")
        .set_json(&new_user)
        .to_request();

    let resp = create_user::create_user(req, config).await;
    assert!(resp.status().is_success());
}

#[actix_web::test]
async fn test_update_user() {
    // Prueba para verificar la actualización de un usuario
}

