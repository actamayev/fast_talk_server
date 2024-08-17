use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpResponse, HttpMessage, web::Data, body::BoxBody
};
use sea_orm::DatabaseConnection;
use futures::future::{ok, LocalBoxFuture, Ready};
use serde_json::json;
use std::rc::Rc;
use std::task::{Context, Poll};

use crate::db::read::credentials::find_user_by_id;
use crate::types::globals::FriendUser;

pub struct ValidateFriendId {
    db: Data<DatabaseConnection>
}

impl ValidateFriendId {
    pub fn new(db: Data<DatabaseConnection>) -> Self {
        ValidateFriendId { db }
    }
}

impl<S> Transform<S, ServiceRequest> for ValidateFriendId
where
    S: Service<ServiceRequest, Response = ServiceResponse<BoxBody>, Error = Error> + 'static,
{
    type Response = ServiceResponse<BoxBody>;
    type Error = Error;
    type Transform = ValidateFriendIdMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(ValidateFriendIdMiddleware {
            service: Rc::new(service),
            db: self.db.clone()
        })
    }
}

pub struct ValidateFriendIdMiddleware<S> {
    service: Rc<S>,
    db: Data<DatabaseConnection>
}

impl<S> Service<ServiceRequest> for ValidateFriendIdMiddleware<S>
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
            // Extract the friendId from the path
            if let Some(friend_id_str) = req.match_info().get("friendId") {
                // Attempt to parse friend_id_str as an i32
                if let Ok(friend_id) = friend_id_str.parse::<i32>() {
                    // Find the user by friend_id
                    if let Some(user) = find_user_by_id(&db, friend_id).await? {
                        // Attach the user to the request's extensions for future access
                        req.extensions_mut().insert(FriendUser(user));

                        // Proceed to the next service
                        return service.call(req).await;
                    } else {
                        // If the user is not found, return a 404 response
                        let response = HttpResponse::NotFound()
                            .json(json!({"message": "User not found"}))
                            .map_into_boxed_body();
                        return Ok(ServiceResponse::new(req.into_parts().0, response));
                    }
                }
            }

            // If friendId is missing or not a valid number, return an error response
            let response = HttpResponse::BadRequest()
                .json(json!({"message": "Invalid or missing friendId in URL parameters"}))
                .map_into_boxed_body();
            Ok(ServiceResponse::new(req.into_parts().0, response))
        })
    }
}
