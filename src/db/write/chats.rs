use chrono::{Utc, TimeZone, FixedOffset};
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};
use std::error::Error;
use crate::entities::chats;

pub async fn add_chats_record(db: &DatabaseConnection) -> Result<i32, Box<dyn Error>> {
    // Get the current Utc time
    let now_utc = Utc::now();

    // Convert Utc time to a FixedOffset with a zero offset (UTC)
    let now_fixed = FixedOffset::east_opt(0)
        .ok_or("Failed to create FixedOffset")?
        .from_utc_datetime(&now_utc.naive_utc());

    // Create a new ActiveModel instance for the chats table
    let chats = chats::ActiveModel {
        last_message: Set(None), // Set to None if there is no initial message
        updated_at: Set(now_fixed), // Set the fixed offset time
        ..Default::default()
    };
    // Insert the new record into the database and get the result
    let insert_result = chats.insert(db).await?;

    // Return the chat ID (primary key)
    Ok(insert_result.chat_id)
}
