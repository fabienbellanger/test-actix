use std::pin::Pin;
use std::task::{Context, Poll};

use crate::models::auth;
use actix_service::{Service, Transform};
use actix_web::{
    dev::{ServiceRequest, ServiceResponse},
    http::Method,
    http::StatusCode,
    Error, HttpResponse,
};
use futures::future::{ok, Ready};
use futures::Future;

const AUTHORIZATION: &str = "Authorization";

pub struct Authentication;

impl<S, B> Transform<S> for Authentication
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = AuthenticationMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthenticationMiddleware { service })
    }
}

pub struct AuthenticationMiddleware<S> {
    service: S,
}

impl<S, B> Service for AuthenticationMiddleware<S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, req: ServiceRequest) -> Self::Future {
        let mut auth_success: bool = false;

        if Method::OPTIONS == *req.method() {
            auth_success = true;
        } else {
            if let Some(auth_header) = req.headers().get(AUTHORIZATION) {
                if let Ok(auth_str) = auth_header.to_str() {
                    if auth_str.starts_with("bearer") || auth_str.starts_with("Bearer") {
                        let token = auth_str[6..auth_str.len()].trim();
                        if let Ok(_token_data) = auth::JWT::parse(token.to_owned()) {
                            // TODO: Vérification en BDD
                            auth_success = true;
                        } else {
                            eprintln!("Failed to parse token: {}", token);
                        }
                    }
                }
            }
        }

        if auth_success {
            let fut = self.service.call(req);
            Box::pin(async move {
                let res = fut.await?;
                Ok(res)
            })
        } else {
            Box::pin(async move {
                Ok(req.into_response(
                    HttpResponse::Unauthorized()
                        .json(crate::errors::AppErrorMessage {
                            code: StatusCode::UNAUTHORIZED.as_u16(),
                            error: "Unauthorized".to_owned(),
                            message: "Unauthorized".to_owned(),
                        })
                        .into_body(),
                ))
            })
        }
    }
}
