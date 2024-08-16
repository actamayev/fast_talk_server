//src/handlers/login
use actix_web::{web, HttpResponse, Error};
use sea_orm::DatabaseConnection;
use crate::utils::auth_helpers::{hash::Hash, sign_jwt::sign_jwt};
use crate::types::{incoming_requests::LoginRequest, outgoing_responses::AuthResponse};
use crate::db::{read::credentials::find_user_by_contact, write::login_history::add_login_history};

pub async fn login(
    db: web::Data<DatabaseConnection>, // Inject the DatabaseConnection
    req: web::Json<LoginRequest>
) -> Result<HttpResponse, Error> {
    let user = find_user_by_contact(&db, &req.contact).await?;

    let user = match user {
        Some(user) => user,
        None => {
            return Ok(HttpResponse::Unauthorized().json(serde_json::json!({
                "message": "Invalid credentials"
            })));
        }
    };

    let do_passwords_match = Hash::check_password(&req.password, &user.password)
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    if !do_passwords_match {
        return Ok(HttpResponse::BadRequest().json(serde_json::json!({
            "message": "Wrong password"
        })));
    }

    let access_token = sign_jwt(&user.user_id).await?;

    add_login_history(&db, user.user_id).await?;

    let response = AuthResponse {
        access_token
    };

    Ok(HttpResponse::Ok().json(response))
}
