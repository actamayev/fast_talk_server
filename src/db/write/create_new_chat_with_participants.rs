use std::error::Error;
use chrono::{FixedOffset, TimeZone, Utc};
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set, TransactionTrait};
use crate::entities::{chats, chat_participants::ActiveModel as ChatParticipantActiveModel};

pub async fn create_chat_with_participants(
    db: &DatabaseConnection,
    user_id_1: i32,
    user_id_2: i32,
) -> Result<i32, Box<dyn Error>> {
    // Start a transaction
    let txn = db.begin().await?;

    // Get the current Utc time
    let now_utc = Utc::now();
    let now_fixed = FixedOffset::east_opt(0)
        .ok_or("Failed to create FixedOffset")?
        .from_utc_datetime(&now_utc.naive_utc());

    // Create a new chat record
    let chats = chats::ActiveModel {
        last_message: Set(None), // Set to None if there is no initial message
        updated_at: Set(now_fixed), // Set the fixed offset time
        ..Default::default()
    };

    // Insert the new chat record into the database
    let insert_result = chats.insert(&txn).await?;
    let chat_id = insert_result.chat_id; // Retrieve the chat ID

    // Create two new ActiveModel instances for the chat_participants table
    let chat_participant_1 = ChatParticipantActiveModel {
        chat_id: Set(chat_id),
        user_id: Set(user_id_1),
        joined_at: Set(now_fixed),
        ..Default::default()
    };

    let chat_participant_2 = ChatParticipantActiveModel {
        chat_id: Set(chat_id),
        user_id: Set(user_id_2),
        joined_at: Set(now_fixed),
        ..Default::default()
    };

    // Insert the first chat participant record into the database
    chat_participant_1.insert(&txn).await?;

    // Insert the second chat participant record into the database
    chat_participant_2.insert(&txn).await?;

    // Commit the transaction
    txn.commit().await?;

    // Return the chat ID
    Ok(chat_id)
}
