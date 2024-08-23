use std::error::Error;
use chrono::{FixedOffset, TimeZone, Utc, NaiveDateTime};
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set, TransactionTrait};
use crate::entities::{messages, chats};

pub struct AddMessage {
    pub message_id: i32,
    pub message_sent_time: NaiveDateTime,
}

pub async fn add_message_and_update_chat(
    db: &DatabaseConnection,
    chat_id: i32,
    sender_id: i32,
    text: String,
) -> Result<AddMessage, Box<dyn Error>> {
    // Start a transaction
    let txn = db.begin().await?;

    // Get the current Utc time
    let now_utc = Utc::now();
    let now_naive = now_utc.naive_utc();  // Convert to NaiveDateTime
    let now_fixed = FixedOffset::east_opt(0)
        .ok_or("Failed to create FixedOffset")?
        .from_utc_datetime(&now_utc.naive_utc());

    // Create a new message record
    let new_message = messages::ActiveModel {
        chat_id: Set(chat_id),
        sender_id: Set(sender_id),
        text: Set(text.clone()),  // Clone the text to use it in the next update
        sent_at: Set(now_fixed),  // Use NaiveDateTime
        ..Default::default()
    };

    // Insert the new message record into the database
    let insert_result = new_message.insert(&txn).await?;

    // Update the chat's last message and updated_at fields
    if let Some(chat) = chats::Entity::find_by_id(chat_id).one(&txn).await? {
        let mut chat: chats::ActiveModel = chat.into();
        chat.last_message = Set(Some(text));           // Update the last_message field with the text of the new message
        chat.updated_at = Set(now_fixed);              // Update the updated_at field with the current time
        chat.last_message_sender_id = Set(Some(sender_id));

        chat.update(&txn).await?;
    } else {
        // If the chat record does not exist, roll back the transaction and return an error
        txn.rollback().await?;
        return Err(Box::from("Chat record not found"));
    }

    // Commit the transaction
    txn.commit().await?;

    // Return the AddMessage struct with the message ID and sent time
    let add_message_response = AddMessage {
        message_id: insert_result.message_id,
        message_sent_time: now_naive,  // Use NaiveDateTime
    };

    Ok(add_message_response)
}
