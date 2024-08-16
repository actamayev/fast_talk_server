//src/middleware/base_middleware.rs
use actix_web::{dev::{Service, ServiceRequest, ServiceResponse}, Error};
use futures::future::LocalBoxFuture;
use std::rc::Rc;
use std::task::{Context, Poll};

// Define a reusable base middleware struct
pub struct BaseMiddleware<T> {
    service: Rc<T>,
}

// Implement the base middleware logic
impl<T, B> Service<ServiceRequest> for BaseMiddleware<T>
where
    T: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let service = Rc::clone(&self.service);

        Box::pin(async move {
            // Logic specific to derived middleware would go here
            service.call(req).await
        })
    }
}
