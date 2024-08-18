use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QuerySelect};
use std::error::Error;
use crate::entities::{chat_participants, credentials};

#[derive(Debug)]
pub struct ChatUserInfo {
    pub chat_id: i32,
    pub username: String,
}

pub async fn get_chat_usernames(
    db: &DatabaseConnection,
    chat_ids: &[i32],  // Accept a slice reference instead of a Vec
    user_id: i32,
) -> Result<Vec<ChatUserInfo>, Box<dyn Error>> {
    let chat_ids_vec = chat_ids.to_vec();

    // Step 1: Get the chat participants excluding the current user
    let chat_participants = chat_participants::Entity::find()
        .filter(chat_participants::Column::ChatId.is_in(chat_ids_vec))
        .filter(chat_participants::Column::UserId.ne(user_id)) // Exclude the current user's ID
        .select_only()
        .column(chat_participants::Column::ChatId)
        .column(chat_participants::Column::UserId)
        .into_tuple::<(i32, i32)>() // Tuple of (chat_id, user_id)
        .all(db)
        .await?;
    
    // Step 2: Get the usernames for the participant IDs
    let user_ids: Vec<i32> = chat_participants.iter().map(|(_, user_id)| *user_id).collect();
    
    let user_info = credentials::Entity::find()
        .filter(credentials::Column::UserId.is_in(user_ids))
        .select_only()
        .column(credentials::Column::UserId)
        .column(credentials::Column::Username)
        .into_tuple::<(i32, String)>()
        .all(db)
        .await?;

    // Step 3: Map the usernames back to the chat IDs
    let mut chat_user_info: Vec<ChatUserInfo> = Vec::new();

    for (chat_id, participant_id) in chat_participants {
        if let Some((_, username)) = user_info.iter().find(|(user_id, _)| *user_id == participant_id) {
            chat_user_info.push(ChatUserInfo {
                chat_id,
                username: username.clone(),
            });
        }
    }

    Ok(chat_user_info)
}
