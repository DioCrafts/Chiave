// src/middleware/auth_middleware.rs
use actix_web::{dev::ServiceRequest, dev::ServiceResponse, Error};
use actix_web::http::header::HeaderValue;
use actix_web::dev::Transform;
use futures::future::{ok, Ready};

pub struct AuthMiddleware;

impl<S> Transform<S> for AuthMiddleware
where
    S: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthMiddlewareService { service })
    }
}

pub struct AuthMiddlewareService<S> {
    service: S,
}

impl<S> actix_service::Service for AuthMiddlewareService<S>
where
    S: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse;
    type Error = Error;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn call(&self, req: ServiceRequest) -> Self::Future {
        if let Some(auth_header) = req.headers().get("Authorization") {
            // Lógica de validación del token JWT
        } else {
            // Retornar error si no se proporciona el token
        }
        self.service.call(req)
    }
}

