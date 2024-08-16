use serde_json::json;
use sea_orm::DatabaseConnection;
use actix_web::{web, Error, HttpRequest, HttpMessage, HttpResponse};
use crate::db::write::{chat_participants::add_chat_participants_record, chats::add_chats_record};
use crate::types::{globals::{AuthenticatedUser, FriendUser}, outgoing_responses::CreateChatResponse}; 

pub async fn create_chat(
    db: web::Data<DatabaseConnection>,
    req: HttpRequest,
) -> Result<HttpResponse, Error> {
    // Extract the authenticated user from the request extensions
    let user = match req.extensions().get::<AuthenticatedUser>().cloned() {
        Some(AuthenticatedUser(user)) => user,
        None => {
            return Ok(HttpResponse::Unauthorized().json(json!({"message": "User not found"})));
        }
    };

    // Extract the friend user from the request extensions
    let friend = match req.extensions().get::<FriendUser>().cloned() {
        Some(FriendUser(friend)) => friend,
        None => {
            return Ok(HttpResponse::Unauthorized().json(json!({"message": "Friend not found"})));
        }
    };

    // Create a new chat record
    let chat_id = match add_chats_record(&db).await {
        Ok(id) => id,
        Err(e) => {
            return Ok(HttpResponse::InternalServerError().json(json!({"message": "Failed to create chat", "error": e.to_string()})));
        }
    };

    // Add chat participants
    if let Err(e) = add_chat_participants_record(&db, user.user_id, friend.user_id, chat_id).await {
        return Ok(HttpResponse::InternalServerError().json(json!({"message": "Failed to add chat participants", "error": e.to_string()})));
    }

    // Return success response
    let response = CreateChatResponse { chat_id };
    Ok(HttpResponse::Ok().json(response))
}
