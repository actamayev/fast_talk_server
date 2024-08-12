use actix_web::{web, HttpRequest, HttpResponse, Result, FromRequest};
use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::Error;
use futures::future::{ok, Ready};
use futures::Future;
use std::pin::Pin;
use validator::{Validate, ValidationError};
use serde::Deserialize;
use actix_web::body::{EitherBody, BoxBody};

// Define the structs with validation
#[derive(Debug, Validate, Deserialize)]
pub struct LoginInformation {
    #[validate(length(min = 3, max = 100))]
    contact: String,

    #[validate(length(min = 6, max = 100))]
    password: String,
}

#[derive(Debug, Validate, Deserialize)]
pub struct LoginRequest {
    #[validate]
    login_information: LoginInformation,
}

// Middleware for validating the login request
pub struct ValidateLogin;

impl<S, B> Transform<S, ServiceRequest> for ValidateLogin
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + Clone + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B, BoxBody>>;
    type Error = Error;
    type Transform = ValidateLoginMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(ValidateLoginMiddleware { service })
    }
}

pub struct ValidateLoginMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for ValidateLoginMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + Clone + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B, BoxBody>>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(
        &self,
        ctx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let (http_req, mut payload) = req.into_parts();
        let service = self.service.clone();

        Box::pin(async move {
            // Deserialize the body into LoginRequest
            let body = web::Json::<LoginRequest>::from_request(&http_req, &mut payload).await;

            // Handle validation
            match body {
                Ok(body) => {
                    if let Err(validation_error) = body.validate() {
                        return Ok(ServiceResponse::new(
                            http_req,
                            HttpResponse::BadRequest()
                                .json(format!("Validation error: {:?}", validation_error))
                                .map_into_right_body(),  // Convert to right body type BoxBody
                        ));
                    }

                    // Replace the request with validated body
                    let req = ServiceRequest::from_parts(http_req, payload);
                    service
                        .call(req)
                        .await
                        .map(ServiceResponse::map_into_left_body)  // Convert to left body type B
                }
                Err(_) => Ok(ServiceResponse::new(
                    http_req,
                    HttpResponse::BadRequest()
                        .json("Invalid request format")
                        .map_into_right_body(),  // Convert to right body type BoxBody
                )),
            }
        })
    }
}
