use std::future::{ready, Future, Ready};
use std::pin::Pin;

use actix_web::HttpResponse;
use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error,
};

use actix_web::error::ErrorUnauthorized;
use actix_web::http::header;
use log::trace;
use reqwest::header::HeaderValue;

pub struct CookieParser {}

impl Default for CookieParser {
    fn default() -> Self {
        Self::new()
    }
}

impl CookieParser {
    pub fn new() -> Self {
        CookieParser {}
    }
}

impl<S, B> Transform<S, ServiceRequest> for CookieParser
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = CookieParserMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(CookieParserMiddleware { service }))
    }
}

pub struct CookieParserMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for CookieParserMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    forward_ready!(service);

    fn call(&self, mut req: ServiceRequest) -> Self::Future {
        let cookie_val = req
            .cookie("bearer_token")
            .map(|cookie| cookie.value().to_owned())
            .ok_or_else(|| HttpResponse::Unauthorized().finish());
        if cookie_val.is_err() {
            return Box::pin(async move { Err(ErrorUnauthorized("Failed to parse cookie")) });
        }
        let cookie = cookie_val.expect("Should be valid");
        let auth_header_value = format!("Bearer {}", cookie);
        let header_value = HeaderValue::from_str(auth_header_value.as_str());
        if header_value.is_err() {
            return Box::pin(async move { Err(ErrorUnauthorized("Failed to parse header")) });
        }
        req.headers_mut().insert(
            header::AUTHORIZATION,
            header_value.expect("Should be some."),
        );
        trace!("Initialize Cookie Transform Middleware.");
        let fut = self.service.call(req);

        Box::pin(async move {
            let res = fut.await?;
            Ok(res)
        })
    }
}
