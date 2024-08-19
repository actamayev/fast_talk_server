use serde_json::json;
use sea_orm::DatabaseConnection;
use actix_web::{web, Error, HttpRequest, HttpMessage, HttpResponse};
use crate::db::read::chats::does_chat_exist;
use crate::db::write::add_new_message_and_update_last_message::add_message_and_update_chat;
use crate::types::globals::AuthenticatedUser;
use crate::types::incoming_requests::NewMessageRequest;
use crate::db::read::chat_participants::is_user_in_chat;
use crate::types::outgoing_responses::SendMessageResponse;

pub async fn send_message(
    db: web::Data<DatabaseConnection>,
    req: HttpRequest,  // Use HttpRequest to access extensions
    path: web::Path<i32>, // Extract chatId from the path
    json: web::Json<NewMessageRequest>, // Extract and validate the incoming JSON
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

	let message_id = add_message_and_update_chat(&db, chat_id, user.user_id, json.message.clone()).await?;

    // Return success response
    let response = SendMessageResponse { message_id };
    Ok(HttpResponse::Ok().json(response))
}
