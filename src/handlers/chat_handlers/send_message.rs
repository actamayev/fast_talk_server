use serde_json::json;
use sea_orm::DatabaseConnection;
use actix_web::{web, Error, HttpRequest, HttpMessage, HttpResponse};
use crate::db::read::chats::does_chat_exist;
use crate::types::globals::AuthenticatedUser;
use crate::types::incoming_requests::NewMessageRequest;
use crate::utils::socket::socket_setup::{ClientMap, WsMessage};
use crate::db::read::chat_participants::{get_other_user_in_chat, is_user_in_chat};
use crate::types::outgoing_responses::{OutgoingSocketMessage, SendMessageResponse};
use crate::db::write::add_new_message_and_update_chat::add_message_and_update_chat;

pub async fn send_message(
    db: web::Data<DatabaseConnection>,
    req: HttpRequest,  // Use HttpRequest to access extensions
    path: web::Path<i32>, // Extract chatId from the path
    json: web::Json<NewMessageRequest>, // Extract and validate the incoming JSON
    clients: web::Data<ClientMap>,
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

	let new_message_response = add_message_and_update_chat(&db, chat_id, user.user_id, json.message.clone()).await?;

    let other_user_details = match get_other_user_in_chat(&db, chat_id, user.user_id).await {
        Ok(Some(other_user_details)) => other_user_details, // Extract the user_id if present
        Ok(None) => {
            return Ok(HttpResponse::InternalServerError().json(json!({"message": "Unable to find other user"})));
        }
        Err(e) => {
            return Ok(HttpResponse::InternalServerError().json(json!({"message": "Failed to check if chat exists", "error": e.to_string()})));
        }
    };

    let clients = clients.lock().unwrap();
    if let Some(addr) = clients.get(&other_user_details.user_id) {
        let outgoing_socket_response = OutgoingSocketMessage {
            chat_id,
            message_id: new_message_response.message_id, 
            message_text: json.message.clone(),
            sent_time: new_message_response.message_sent_time,
            message_sender_user_id: user.user_id,
            message_sender_username: user.username
        };
        if let Ok(serialized_message) = serde_json::to_string(&outgoing_socket_response) {
            addr.do_send(WsMessage(serialized_message)); // Send the serialized message to the WebSocket
        } else {
            eprintln!("Failed to serialize OutgoingSocketMessage");
        }
    }

    // Return success response
    let response = SendMessageResponse { message_id: new_message_response.message_id };
    Ok(HttpResponse::Ok().json(response))
}
