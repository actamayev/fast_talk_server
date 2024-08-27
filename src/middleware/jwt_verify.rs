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
    db: Data<DatabaseConnection>
}

impl JwtVerify {
    pub fn new(db: Data<DatabaseConnection>) -> Self {
        Self {
            cache: Arc::new(RwLock::new(HashMap::new())),
            db,
        }
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
            cache: self.cache.clone(),
            db: self.db.clone(),
        }))
    }
}

impl<S> AuthMiddleware<S> {
    async fn get_user_from_cache(
        &self,
        user_id: i32,
    ) -> Result<AuthenticatedUser, ()> {
        let cache = self.cache.write().await;
    
        // Check if the user is in the cache and not expired
        if let Some((user, timestamp)) = cache.get(&user_id) {
            if timestamp.elapsed() < Duration::from_secs(600) {
                println!("here, user exists in cache");
                Ok(user.clone())
            } else {
                Err(())
            }
        } else {
            Err(())
        }
    }

    async fn get_user_from_db_and_cache(
        &self,
        user_id: i32,
    ) -> Result<AuthenticatedUser, actix_web::Error> {
        let user_result = find_user_by_id(&self.db, user_id).await;
    
        // Handle the result, returning an error if the user is not found
        let user = match user_result {
            Ok(Some(user)) => user,
            Ok(None) => {
                let response = HttpResponse::Unauthorized()
                    .json(json!({"message": "User not found"}));
                return Err(actix_web::Error::from(InternalError::from_response("", response)));
            }
            Err(err) => {
                return Err(err.into());
            }
        };
    
        // Cache the user
        let authenticated_user = AuthenticatedUser(user.clone());
        {
            let mut cache = self.cache.write().await;
            cache.insert(user_id, (authenticated_user.clone(), Instant::now()));
        }
    
        Ok(authenticated_user)
    }

    async fn authenticate_user(
        &self,
        user_id: i32,
    ) -> Result<AuthenticatedUser, actix_web::Error> {
        match self.get_user_from_cache(user_id).await {
            Ok(user) => Ok(user),
            Err(_) => self.get_user_from_db_and_cache(user_id).await,
        }
    }
}

pub struct AuthMiddleware<S> {
    service: Rc<S>,
    cache: Arc<RwLock<HashMap<i32, (AuthenticatedUser, Instant)>>>,
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
        let self_rc: Rc<AuthMiddleware<S>> = Rc::clone(&self); // This works because `self` is now `Rc<AuthMiddleware<S>>`

        Box::pin(async move {
            if let Some(auth_header) = req.headers().get("Authorization") {
                if let Ok(auth_str) = auth_header.to_str() {
                    let claims = decode_jwt(auth_str)?;
                    let user_id = claims.sub.parse::<i32>().map_err(|_| {
                        let response = HttpResponse::Unauthorized()
                            .json(json!({"message": "Invalid user ID in token"}));
                        actix_web::Error::from(InternalError::from_response("", response))
                    })?;

                    // Dereference Rc to call the method on AuthMiddleware<S>
                    let user = self_rc.authenticate_user(user_id).await?;
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
