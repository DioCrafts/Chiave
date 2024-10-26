// integration_tests/test_auth.rs
use actix_web::test;
use auth_service::{handlers::login, config::Config};

#[actix_web::test]
async fn test_login_success() {
    let config = Config {
        jwt_secret: "test_secret".to_string(),
        ..Default::default()
    };
    let credentials = json!({"username": "test_user", "password": "test_password"});
    
    let req = test::TestRequest::post()
        .uri("/login")
        .set_json(&credentials)
        .to_request();
    
    let resp = login::login_handler(req, config).await;
    assert!(resp.status().is_success());
}

#[actix_web::test]
async fn test_login_failure() {
    // Prueba para validar el fallo de login con credenciales incorrectas
}

