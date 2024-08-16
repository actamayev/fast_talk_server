use actix_web::{web, Error, HttpRequest, HttpMessage, HttpResponse};
use sea_orm::DatabaseConnection;
use crate::types::globals::{AuthenticatedUser, FriendUser}; // Assuming User is the type of your user model
use serde_json::json;

pub async fn create_chat(
    _db: web::Data<DatabaseConnection>,
    req: HttpRequest,                // Access the HttpRequest to get the user
) -> Result<HttpResponse, Error> {
    if let Some(AuthenticatedUser(user)) = req.extensions().get::<AuthenticatedUser>() {
        if let Some(FriendUser(friend)) = req.extensions().get::<FriendUser>() {
            // Now you have access to the user and friend_id
            println!("User ID: {}, Friend ID: {}", user.user_id, friend.user_id);

            // Proceed with your logic, e.g., create a chat between the user and friend_id
            // Example:
            // let chat_id = create_chat_in_db(&db, user.user_id, friend_id.into_inner(), body.into_inner()).await?;

            Ok(HttpResponse::Ok().json(json!({
                "message": "Chat created",
                "user_id": user.user_id,
                "friend_id": friend.user_id,
                // "chat_id": chat_id, // If you return a chat ID
            })))
        }
        else {
            // Handle the case where the user is not found (shouldn't happen if middleware works correctly)
            Ok(HttpResponse::Unauthorized().json(json!({"message": "friend not found"})))
        }
    } else {
        // Handle the case where the user is not found (shouldn't happen if middleware works correctly)
        Ok(HttpResponse::Unauthorized().json(json!({"message": "User not found"})))
    }
}
