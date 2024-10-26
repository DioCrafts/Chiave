// src/handlers/oauth.rs
use actix_web::{get, HttpResponse, Responder};
use crate::config::Config;

#[get("/oauth/login")]
async fn oauth_login(config: web::Data<Config>) -> impl Responder {
    let redirect_url = format!(
        "https://oauthprovider.com/auth?client_id={}&redirect_uri={}&response_type=code",
        config.oauth_client_id, config.oauth_redirect_url
    );
    HttpResponse::Found().header("Location", redirect_url).finish()
}

