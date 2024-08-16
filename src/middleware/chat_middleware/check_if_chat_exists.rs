use std::rc::Rc;
use serde_json::json;
use std::task::{Context, Poll};
use sea_orm::DatabaseConnection;
use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpResponse, HttpMessage, web::Data, body::BoxBody
};
use futures::future::{ok, LocalBoxFuture, Ready};

use crate::{db::read::chat_participants::does_existing_chat_exist, types::globals::{AuthenticatedUser, FriendUser}};

pub struct CheckIfChatExists {
    db: Data<DatabaseConnection>
}

impl CheckIfChatExists {
    pub fn new(db: Data<DatabaseConnection>) -> Self {
        CheckIfChatExists { db }
    }
}

impl<S> Transform<S, ServiceRequest> for CheckIfChatExists
where
    S: Service<ServiceRequest, Response = ServiceResponse<BoxBody>, Error = Error> + 'static,
{
    type Response = ServiceResponse<BoxBody>;
    type Error = Error;
    type Transform = CheckIfChatExistsMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(CheckIfChatExistsMiddleware {
            service: Rc::new(service),
            db: self.db.clone()
        })
    }
}

pub struct CheckIfChatExistsMiddleware<S> {
    service: Rc<S>,
    db: Data<DatabaseConnection>
}

impl<S> Service<ServiceRequest> for CheckIfChatExistsMiddleware<S>
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
			// Extract the necessary extensions before proceeding
			let user = req.extensions().get::<AuthenticatedUser>().cloned();
			let friend = req.extensions().get::<FriendUser>().cloned();
	
			// Handle the case where the extensions are found
			if let (Some(AuthenticatedUser(user)), Some(FriendUser(friend))) = (user, friend) {
				// Check if a chat already exists between the two users
				let chat_exists = does_existing_chat_exist(&db, user.user_id, friend.user_id)
					.await
					.map_err(actix_web::error::ErrorInternalServerError)?;
	
				if chat_exists {
					let response = HttpResponse::Conflict()  // Use Conflict (409) instead of NotFound (404)
						.json(json!({"message": "Chat already exists"}))
						.map_into_boxed_body();
					return Ok(ServiceResponse::new(req.into_parts().0, response));
				} else {
					// Proceed to the next middleware or handler
					return service.call(req).await;
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
