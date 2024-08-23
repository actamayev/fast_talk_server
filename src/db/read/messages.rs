use crate::{entities::messages, types::outgoing_responses::ChatMessage};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder};

pub async fn get_chat_messages(
    db: &DatabaseConnection,
    chat_id: i32,
    user_id: i32,
) -> Result<Vec<ChatMessage>, Box<dyn std::error::Error>> {
    let messages = messages::Entity::find()
        .filter(messages::Column::ChatId.eq(chat_id)) // Filter by chat_id
        .order_by_asc(messages::Column::SentAt) // Order by sent time
        .all(db)
        .await?;

    let chat_messages: Vec<ChatMessage> = messages.into_iter()
        .map(|msg| ChatMessage {
            message_id: msg.message_id,
            did_user_send: msg.sender_id == user_id,
            sender_user_id: msg.sender_id,
            message_text: msg.text,
            sent_time: msg.sent_at.naive_utc(), // Convert to NaiveDateTime if needed
        })
        .collect();

    Ok(chat_messages)
}
