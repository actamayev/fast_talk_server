use std::rc::Rc;
use std::sync::Arc;
use serde_json::json;
use tokio::sync::RwLock;
use std::collections::HashMap;
use std::task::{Context, Poll};
use sea_orm::DatabaseConnection;
use std::time::{Duration, Instant};
use futures::future::{ok, LocalBoxFuture, Ready};
use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpResponse, HttpMessage, web::Data, body::BoxBody,
    error::InternalError
};
use crate::types::globals::AuthenticatedUser;
use crate::{db::read::credentials::find_user_by_id, utils::auth_helpers::jwt::decode_jwt};

#[derive(Clone)]
pub struct JwtVerify {
    cache: Arc<RwLock<HashMap<i32, (AuthenticatedUser, Instant)>>>,
    db: Data<DatabaseConnection>, // Replace with your actual database connection type
}

impl JwtVerify {
    pub fn new(db: Data<DatabaseConnection>) -> Self {
        Self {
            cache: Arc::new(RwLock::new(HashMap::new())),
            db,
        }
    }

    pub async fn get_user(&self, user_id: i32) -> Result<AuthenticatedUser, Error> {
        let mut cache = self.cache.write().await;

        // Check if the user is in the cache and not expired
        if let Some((user, timestamp)) = cache.get(&user_id) {
            if timestamp.elapsed() < Duration::from_secs(600) {
                println!("here, user exists in cache");
                return Ok(user.clone());
            }
        }

        // User is not in cache or entry is expired, fetch from the database
        let user = find_user_by_id(&self.db, user_id).await?.ok_or_else(|| {
            let response = HttpResponse::Unauthorized()
                .json(json!({"message": "User not found"}));
            actix_web::Error::from(InternalError::from_response("", response))
        })?;

        // Cache the user
        cache.insert(user_id, (AuthenticatedUser(user.clone()), Instant::now()));
        println!("user doesn't exist in cache, attaching now");

        Ok(AuthenticatedUser(user))
    }
}

impl<S> Transform<S, ServiceRequest> for JwtVerify
where
    S: Service<ServiceRequest, Response = ServiceResponse<BoxBody>, Error = Error> + 'static,
{
    type Response = ServiceResponse<BoxBody>;
    type Error = Error;
    type Transform = AuthMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthMiddleware {
            service: Rc::new(service),
            cache: self.cache.clone(),
            db: self.db.clone(),
        })
    }
}

pub struct AuthMiddleware<S> {
    service: Rc<S>,
    cache: Arc<RwLock<HashMap<i32, (AuthenticatedUser, Instant)>>>,
    db: Data<DatabaseConnection>, // Replace with your actual database connection type
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
        let cache = self.cache.clone();
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

                    let auth_cache = JwtVerify { cache, db };
                    let user = auth_cache.get_user(user_id).await?;

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
