use serde_json::json;
use sea_orm::DatabaseConnection;
use actix_web::{web, Error, HttpRequest, HttpMessage, HttpResponse};
use crate::db::read::chats::get_chats_info;
use crate::types::globals::AuthenticatedUser;
use crate::db::read::chat_participants::get_user_chat_ids;
use crate::types::outgoing_responses::SingleRetrievedChat;
use crate::db::read::retrieve_chat_usernames::retrieve_chat_usernames;

pub async fn retrieve_chats_list(
    db: web::Data<DatabaseConnection>,
    req: HttpRequest
) -> Result<HttpResponse, Error> {
    // Extract the authenticated user from the request extensions
    let user = match req.extensions().get::<AuthenticatedUser>().cloned() {
        Some(AuthenticatedUser(user)) => user,
        None => {
            return Ok(HttpResponse::Unauthorized().json(json!({"message": "User not found"})));
        }
    };

	let chat_ids = get_user_chat_ids(&db, user.user_id).await.map_err(|e| {
		let error_message = format!("Failed to retrieve chat IDs for user with ID {}: {}", user.user_id, e);
		let error: actix_web::Error = actix_web::error::InternalError::from_response(
			error_message.clone(),
			HttpResponse::InternalServerError().body(error_message)
		).into();
		error
	})?;
	
	let chat_info = get_chats_info(&db, &chat_ids).await.map_err(|e| {
		let error_message = format!("Failed to retrieve chat information for chat IDs {:?}: {}", chat_ids, e);
		let error: actix_web::Error = actix_web::error::InternalError::from_response(
			error_message.clone(),
			HttpResponse::InternalServerError().body(error_message)
		).into();
		error
	})?;

	let chat_usernames = retrieve_chat_usernames(&db, &chat_ids, user.user_id).await.map_err(|e| {
		let error_message = format!("Failed to retrieve chat usernames for user ID {} and chat IDs {:?}: {}", user.user_id, chat_ids, e);
		let error: actix_web::Error = actix_web::error::InternalError::from_response(
			error_message.clone(),
			HttpResponse::InternalServerError().body(error_message)
		).into();
		error
	})?;

	// Calculate the combined chats
	let combined_chats: Vec<SingleRetrievedChat> = chat_info.into_iter()
		.filter_map(|chat| {
			chat_usernames.iter().find(|user_info| user_info.chat_id == chat.chat_id).map(|user_info| {
				SingleRetrievedChat {
					chat_id: chat.chat_id,
					friend_username: user_info.username.clone(),
					last_message: chat.last_message.clone().unwrap_or_default(),
					last_message_time: chat.updated_at,
					chat_created_at: chat.created_at
				}
			})
		})
		.collect();

    Ok(HttpResponse::Ok().json(combined_chats))
}
