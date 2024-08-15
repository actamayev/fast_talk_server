//src/middleware/register_middleware.rs
use actix_web::{dev::{Service, ServiceRequest, ServiceResponse, Transform}, Error};
use futures::future::Ready;
use crate::middleware::middleware_templates::base_middleware::{MiddlewareTransform, BaseMiddleware};

pub struct ValidateRegister;

impl MiddlewareTransform for ValidateRegister {}

impl<S, B> Transform<S, ServiceRequest> for ValidateRegister
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = BaseMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        <Self as MiddlewareTransform>::new_transform(service)
    }
}
