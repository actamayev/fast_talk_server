use sea_orm::DatabaseConnection;
use actix_web::{web, HttpResponse, Error};
use crate::types::outgoing_responses::AuthResponse;
use crate::types::incoming_requests::RegisterRequest;
use crate::utils::auth_helpers::auth_cache::AuthCache;
use crate::utils::auth_helpers::{hash::Hash, jwt::sign_jwt};
use crate::types::globals::{AuthenticatedUser, CredentialsData};
use crate::db::read::credentials::{does_email_exist, does_username_exist};
use crate::db::write::{login_history::add_login_history, credentials::add_credentials_record};

pub async fn register(
    db: web::Data<DatabaseConnection>,
    req: web::Json<RegisterRequest>,
    auth_cache: web::Data<AuthCache>
) -> Result<HttpResponse, Error> {
    if does_email_exist(&db, &req.email).await? {
        return Ok(HttpResponse::BadRequest().json(serde_json::json!({
            "message": "Email already exists"
        })));
    }

    if does_username_exist(&db, &req.username).await? {
        return Ok(HttpResponse::BadRequest().json(serde_json::json!({
            "message": "Username already exists"
        })));
    }

    let hashed_password = Hash::hash_credentials(&req.password)
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let credentials_data = CredentialsData {
        username: req.username.clone(),     // Cloning is necessary here if you need to use req.username later
        hashed_password,                    // No need to clone; already owned
        email: req.email.clone()            // Cloning is necessary here if you need to use req.email later
    };

    let user = add_credentials_record(&db, credentials_data).await?;
    
    let access_token = sign_jwt(&user.user_id)?;

    add_login_history(&db, user.user_id).await?;

    let response = AuthResponse {
        access_token,
        username: req.username.clone()
    };

    auth_cache.store_user(AuthenticatedUser(user)).await;

    Ok(HttpResponse::Ok().json(response))
}
