use std::error::Error;
use chrono::{DateTime, FixedOffset};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QuerySelect};
use crate::entities::chats;

pub async fn does_chat_exist(db: &DatabaseConnection, chat_id: i32) -> Result<bool, Box<dyn Error>> {
    let chat = chats::Entity::find()
        .filter(chats::Column::ChatId.eq(chat_id)) 
        .one(db)
        .await?;

    Ok(chat.is_some())
}

#[derive(Debug)]
pub struct ChatInfo {
    pub chat_id: i32,
    pub updated_at: DateTime<FixedOffset>,
    pub created_at: DateTime<FixedOffset>,
    pub last_message: Option<String>,
    pub last_message_sender_id: Option<i32>
}

pub async fn get_chats_info(
    db: &DatabaseConnection,
    chat_ids: &[i32],  // Accept a slice reference instead of a Vec
) -> Result<Vec<ChatInfo>, Box<dyn Error>> {
    // Convert the slice to Vec<i32>
    let chat_ids_vec = chat_ids.to_vec();

    // Query the chats table for the given chat_ids
    let chats_models = chats::Entity::find()
        .filter(chats::Column::ChatId.is_in(chat_ids_vec)) // Pass the Vec<i32>
        .select_only()
        .column(chats::Column::ChatId)
        .column(chats::Column::UpdatedAt)
        .column(chats::Column::CreatedAt)
        .column(chats::Column::LastMessage)
        .column(chats::Column::LastMessageSenderId)
        .into_model::<chats::Model>() // Use the model type provided by the SeaORM entity
        .all(db)
        .await?;

    // Map the results from the model into your custom ChatInfo struct
    let chats_info: Vec<ChatInfo> = chats_models
        .into_iter()
        .map(|chat| ChatInfo {
            chat_id: chat.chat_id,
            created_at: chat.created_at,
            updated_at: chat.updated_at,
            last_message: chat.last_message,
            last_message_sender_id: chat.last_message_sender_id
        })
        .collect();

    Ok(chats_info)
}