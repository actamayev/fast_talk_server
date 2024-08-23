use std::error::Error;
use chrono::{FixedOffset, Local, DateTime};
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set, TransactionTrait};
use crate::entities::{messages, chats};

pub struct AddMessage {
    pub message_id: i32,
    pub message_sent_time: DateTime<FixedOffset>
}

pub async fn add_message_and_update_chat(
    db: &DatabaseConnection,
    chat_id: i32,
    sender_id: i32,
    text: String,
) -> Result<AddMessage, Box<dyn Error>> {
    // Start a transaction
    let txn = db.begin().await?;

    // Get the current local time (assumed to be ET)
    let now_local = Local::now();
    let now_fixed: DateTime<FixedOffset> = now_local.with_timezone(&FixedOffset::east_opt(-5 * 3600).unwrap());

    // Create a new message record
    let new_message = messages::ActiveModel {
        chat_id: Set(chat_id),
        sender_id: Set(sender_id),
        text: Set(text.clone()),  // Clone the text to use it in the next update
        sent_at: Set(now_fixed),
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
        message_sent_time: now_fixed
    };

    Ok(add_message_response)
}
