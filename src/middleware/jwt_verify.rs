use std::rc::Rc;
use serde_json::json;
use std::task::{Context, Poll};
use sea_orm::DatabaseConnection;
use futures::future::{ok, LocalBoxFuture, Ready};
use actix_web::{
    body::BoxBody, dev::{Service, ServiceRequest, ServiceResponse, Transform},
    error::InternalError, web::{self, Data}, Error, HttpMessage, HttpResponse
};
use crate::{db::read::credentials::find_user_by_id, utils::auth_helpers::jwt::decode_jwt};
use crate::{types::globals::AuthenticatedUser, utils::auth_helpers::auth_cache::AuthCache};

#[derive(Clone)]
pub struct JwtVerify {
    db: Data<DatabaseConnection>
}

impl JwtVerify {
    pub fn new(db: Data<DatabaseConnection>) -> Self {
        Self { db }
    }
}

impl<S> Transform<S, ServiceRequest> for JwtVerify
where
    S: Service<ServiceRequest, Response = ServiceResponse<BoxBody>, Error = Error> + 'static,
{
    type Response = ServiceResponse<BoxBody>;
    type Error = Error;
    type Transform = Rc<AuthMiddleware<S>>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(Rc::new(AuthMiddleware {
            service: Rc::new(service),
            db: self.db.clone(),
        }))
    }
}

pub struct AuthMiddleware<S> {
    service: Rc<S>,
    db: Data<DatabaseConnection>
}
impl<S> Service<ServiceRequest> for AuthMiddleware<S>
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
        let auth_cache = req.app_data::<web::Data<AuthCache>>().unwrap().clone();
        let db = self.db.clone();

        Box::pin(async move {
            if let Some(auth_header) = req.headers().get("Authorization") {
                if let Ok(auth_str) = auth_header.to_str() {
                    let claims = decode_jwt(auth_str)?;
                    let user_id = claims.sub.parse::<i32>().map_err(|_| {
                        let response = HttpResponse::Unauthorized()
                            .json(json!({"message": "Invalid user ID in token"}));
                        actix_web::Error::from(InternalError::from_response("", response))
                    })?;

                    // Try to get the user from the cache
                    let user = if let Some(user) = auth_cache.get_user(user_id).await {
                        user
                    } else {
                        // If not in cache, fetch from DB and store in cache
                        let user_result = find_user_by_id(&db, user_id).await;
                        match user_result {
                            Ok(Some(user)) => {
                                let authenticated_user = AuthenticatedUser(user.clone());
                                auth_cache.store_user(authenticated_user.clone()).await;
                                authenticated_user
                            }
                            Ok(None) => {
                                let response = HttpResponse::Unauthorized()
                                    .json(json!({"message": "User not found"}));
                                return Err(actix_web::Error::from(InternalError::from_response("", response)));
                            }
                            Err(err) => return Err(err.into()),
                        }
                    };

                    req.extensions_mut().insert(user);

                    return service.call(req).await;
                }
            }

            let response = HttpResponse::Unauthorized()
                .json(json!({"message": "Authorization token is missing or invalid"}))
                .map_into_boxed_body();
            Ok(ServiceResponse::new(req.into_parts().0, response))
        })
    }
}
