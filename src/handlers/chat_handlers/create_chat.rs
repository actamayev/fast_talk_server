use actix_web::{web, HttpResponse, Error};
use sea_orm::DatabaseConnection;
use crate::types::incoming_requests::CreateChat;
use crate::types::outgoing_responses::CreateChatResponse;
use crate::utils::auth_helpers::{hash::Hash, jwt::sign_jwt};
use crate::db::{read::credentials::find_user_by_contact, write::login_history::add_login_history};

pub async fn create_chat(
    db: web::Data<DatabaseConnection>,
    req: web::Json<CreateChat>
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
        .map_err(actix_web::error::ErrorInternalServerError)?;

    if !do_passwords_match {
        return Ok(HttpResponse::BadRequest().json(serde_json::json!({
            "message": "Wrong password"
        })));
    }

    let access_token = sign_jwt(&user.user_id)?;

    add_login_history(&db, user.user_id).await?;

    let response = CreateChatResponse {
        access_token
    };

    Ok(HttpResponse::Ok().json(response))
}
