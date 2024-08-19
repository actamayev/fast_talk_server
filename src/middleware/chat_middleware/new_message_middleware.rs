use std::rc::Rc;
use actix_http::h1;
use serde_json::json;
use validator::Validate;
use actix_web::web::Bytes;
use futures::future::{ok, LocalBoxFuture, Ready};
use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse, Payload, Transform},
    Error, HttpResponse, web, body::BoxBody,
};

use crate::types::incoming_requests::NewMessageRequest;

pub struct ValidateNewMessage;

impl<S> Transform<S, ServiceRequest> for ValidateNewMessage
where
    S: Service<ServiceRequest, Response = ServiceResponse<BoxBody>, Error = Error> + 'static,
{
    type Response = ServiceResponse<BoxBody>;
    type Error = Error;
    type Transform = ValidateNewMessageMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(ValidateNewMessageMiddleware {
            service: Rc::new(service),
        })
    }
}

pub struct ValidateNewMessageMiddleware<S> {
    service: Rc<S>,
}

impl<S> Service<ServiceRequest> for ValidateNewMessageMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<BoxBody>, Error = Error> + 'static,
{
    type Response = ServiceResponse<BoxBody>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&self, _cx: &mut std::task::Context<'_>) -> std::task::Poll<Result<(), Self::Error>> {
        std::task::Poll::Ready(Ok(()))
    }

    fn call(&self, mut req: ServiceRequest) -> Self::Future {
        let service = Rc::clone(&self.service);

        Box::pin(async move {
            // Extract the JSON body
            let payload = req.extract::<web::Json<NewMessageRequest>>().await;

            match payload {
                Ok(body) => {
                    // Validate the extracted data
                    if let Err(e) = body.validate() {
                        let error_message = e.to_string();
                        let response = HttpResponse::BadRequest()
                            .json(json!({"message": error_message}))
                            .map_into_boxed_body(); // Convert to `BoxBody`
                        return Ok(ServiceResponse::new(req.into_parts().0, response));
                    }
                    // Convert the validated LoginRequest back into JSON bytes
                    let json_bytes = serde_json::to_vec(&*body).unwrap();

                    // Create a new payload from the bytes
                    let (mut sender, new_payload) = h1::Payload::create(true);
                    sender.feed_data(Bytes::from(json_bytes));
                    sender.feed_eof();

                    req.set_payload(Payload::from(new_payload));

                    // Proceed to the next service if validation passes
                    service.call(req).await
                }
                Err(_) => {
                    let response = HttpResponse::BadRequest()
                        .json(json!({"message": "Invalid login request body"}))
                        .map_into_boxed_body(); // Convert to `BoxBody`
                    Ok(ServiceResponse::new(req.into_parts().0, response))
                }
            }
        })
    }
}
