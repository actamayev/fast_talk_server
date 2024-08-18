use chrono::{Utc, TimeZone, FixedOffset};
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};
use std::error::Error;
use crate::entities::messages;

pub async fn add_messages_record(
	db: &DatabaseConnection,
	chat_id: i32,
	sender_id: i32,
	text: String
) -> Result<i32, Box<dyn Error>> {
    // Get the current Utc time
    let now_utc = Utc::now();

    // Convert Utc time to a FixedOffset with a zero offset (UTC)
    let now_fixed = FixedOffset::east_opt(0)
        .ok_or("Failed to create FixedOffset")?
        .from_utc_datetime(&now_utc.naive_utc());

    // Create a new ActiveModel instance for the chats table
    let messages = messages::ActiveModel {
		chat_id: Set(chat_id),
		sender_id: Set(sender_id),
		text: Set(text),
		sent_at: Set(now_fixed),
        ..Default::default()
    };
    // Insert the new record into the database and get the result
    let insert_result = messages.insert(db).await?;

    // Return the chat ID (primary key)
    Ok(insert_result.message_id)
}
