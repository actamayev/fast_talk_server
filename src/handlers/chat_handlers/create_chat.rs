use serde_json::json;
use sea_orm::DatabaseConnection;
use actix_web::{web, Error, HttpRequest, HttpMessage, HttpResponse};
use crate::db::write::create_new_chat_with_participants::create_chat_with_participants;
use crate::types::{globals::AuthenticatedUser, outgoing_responses::CreateChatResponse};
use crate::db::read::{chat_participants::does_existing_chat_exist, credentials::find_user_by_id};

pub async fn create_chat(
    db: web::Data<DatabaseConnection>,
    req: HttpRequest,
    path: web::Path<i32>, // Extract friendId from the path
) -> Result<HttpResponse, Error> {
    let friend_id = path.into_inner();

    // Extract the authenticated user from the request extensions
    let user = match req.extensions().get::<AuthenticatedUser>().cloned() {
        Some(AuthenticatedUser(user)) => user,
        None => {
            return Ok(HttpResponse::Unauthorized().json(json!({"message": "User not found"})));
        }
    };

    if user.user_id == friend_id {
        return Ok(HttpResponse::BadRequest().json(json!({"message": "Cannot create a chat with yourself"})));
    }

    // Validate and find the friend user by friend_id
    let friend = match find_user_by_id(&db, friend_id).await {
        Ok(Some(friend)) => friend,
        Ok(None) => {
            return Ok(HttpResponse::NotFound().json(json!({"message": "Friend not found"})));
        }
        Err(e) => {
            return Ok(HttpResponse::InternalServerError().json(json!({"message": "Failed to validate friend ID", "error": e.to_string()})));
        }
    };

    // Check if a chat already exists between the user and the friend
    let chat_exists_result = does_existing_chat_exist(&db, user.user_id, friend.user_id).await;

    match chat_exists_result {
        Ok(true) => {
            return Ok(HttpResponse::Conflict().json(json!({"message": "Chat already exists"})));
        }
        Ok(false) => { }
        Err(e) => {
            return Ok(HttpResponse::InternalServerError().json(json!({"message": "Failed to check if chat exists", "error": e.to_string()})));
        }
    }

    // Create a new chat record
    let chat_id = match create_chat_with_participants(&db, user.user_id, friend.user_id).await {
        Ok(id) => id,
        Err(e) => {
            return Ok(HttpResponse::InternalServerError().json(json!({"message": "Failed to create chat", "error": e.to_string()})));
        }
    };

    // Return success response
    let response = CreateChatResponse { chat_id };
    Ok(HttpResponse::Ok().json(response))
}
