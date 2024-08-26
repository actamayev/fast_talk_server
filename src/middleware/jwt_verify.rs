use std::rc::Rc;
use serde_json::json;
use std::task::{Context, Poll};
use sea_orm::DatabaseConnection;
use futures::future::{ok, LocalBoxFuture, Ready};
use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpResponse, HttpMessage, web::Data, body::BoxBody,
    error::InternalError,
};

use crate::utils::auth_helpers::jwt::decode_jwt;
use crate::{db::read::credentials::find_user_by_id, types::globals::AuthenticatedUser};

#[derive(Clone)]
pub struct JwtVerify {
    db: Data<DatabaseConnection>
}

impl JwtVerify {
    pub fn new(db: Data<DatabaseConnection>) -> Self {
        JwtVerify { db }
    }
}

impl<S> Transform<S, ServiceRequest> for JwtVerify
where
    S: Service<ServiceRequest, Response = ServiceResponse<BoxBody>, Error = Error> + 'static,
{
    type Response = ServiceResponse<BoxBody>;
    type Error = Error;
    type Transform = JwtVerifyMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(JwtVerifyMiddleware {
            service: Rc::new(service),
            db: self.db.clone(),
        })
    }
}

pub struct JwtVerifyMiddleware<S> {
    service: Rc<S>,
    db: Data<DatabaseConnection>,
}

impl<S> Service<ServiceRequest> for JwtVerifyMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<BoxBody>, Error = Error> + 'static,
{
    type Response = ServiceResponse<BoxBody>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let service = Rc::clone(&self.service);
        let db = self.db.clone();

        Box::pin(async move {
            // Extract the Authorization header
            if let Some(auth_header) = req.headers().get("Authorization") {
                if let Ok(auth_str) = auth_header.to_str() {
                    // Decode the JWT and get the user_id
                    let claims = decode_jwt(auth_str)?;
                    let user_id = claims.sub.parse::<i32>().map_err(|_| {
                        // Convert HttpResponse to actix_web::Error
                        let response = HttpResponse::Unauthorized()
                            .json(json!({"message": "Invalid user ID in token"}));
                        actix_web::Error::from(InternalError::from_response("", response))
                    })?;

                    // Find the user by ID
                    let user = find_user_by_id(&db, user_id).await?.expect("User not found");

                    req.extensions_mut().insert(AuthenticatedUser(user));

                    // Proceed to the next service if the header is valid
                    return service.call(req).await;
                }
            }

            // If the Authorization header is missing or invalid, return an error response
            let response = HttpResponse::Unauthorized()
                .json(json!({"message": "Authorization token is missing or invalid"}))
                .map_into_boxed_body();
            Ok(ServiceResponse::new(req.into_parts().0, response))
        })
    }
}
