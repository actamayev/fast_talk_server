use serde_json::json;
use sea_orm::DatabaseConnection;
use actix_web::{web, Error, HttpRequest, HttpMessage, HttpResponse};
use crate::db::read::chats::does_chat_exist;
use crate::types::globals::AuthenticatedUser;
use crate::db::read::messages::get_chat_messages;
use crate::db::read::chat_participants::is_user_in_chat;

pub async fn retrieve_chat_messages(
    db: web::Data<DatabaseConnection>,
    req: HttpRequest,
	path: web::Path<i32>, // Extract chatId from the path
) -> Result<HttpResponse, Error> {
	let chat_id = path.into_inner();

    // Extract the authenticated user from the request extensions
    let user = match req.extensions().get::<AuthenticatedUser>().cloned() {
        Some(AuthenticatedUser(user)) => user,
        None => {
            return Ok(HttpResponse::Unauthorized().json(json!({"message": "User not found"})));
        }
    };

	match does_chat_exist(&db, chat_id).await {
        Ok(false) => return Ok(HttpResponse::Conflict().json(json!({"message": "Chat does not exist"}))),
        Err(e) => return Ok(HttpResponse::InternalServerError().json(json!({"message": "Failed to check if chat exists", "error": e.to_string()}))),
        Ok(true) => {} // Proceed if the chat exists
    }

    match is_user_in_chat(&db, user.user_id, chat_id).await {
        Ok(false) => return Ok(HttpResponse::Conflict().json(json!({"message": "User is not in chat"}))),
        Err(e) => return Ok(HttpResponse::InternalServerError().json(json!({"message": "Failed to check if user is in chat", "error": e.to_string()}))),
        Ok(true) => {} // Proceed if the chat exists
    }

	let chat_messages = get_chat_messages(&db, chat_id, user.user_id).await?;

    Ok(HttpResponse::Ok().json(chat_messages))
}
