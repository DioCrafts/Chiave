// unit_tests/test_jwt.rs
use auth_service::jwt::{create_token, validate_token};

#[test]
fn test_create_token() {
    let secret = "test_secret";
    let username = "test_user";
    let token = create_token(username, secret);
    assert!(!token.is_empty());
}

#[test]
fn test_validate_token() {
    let secret = "test_secret";
    let username = "test_user";
    let token = create_token(username, secret);
    let is_valid = validate_token(&token, secret);
    assert!(is_valid);
}

