use std::error::Error;
use chrono::{Utc, TimeZone, FixedOffset};
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set, TransactionTrait};
use crate::entities::chat_participants::ActiveModel as ChatParticipantActiveModel;

pub async fn add_chat_participants_record(
    db: &DatabaseConnection,
    user_id_1: i32,
    user_id_2: i32,
    chat_id: i32,
) -> Result<(), Box<dyn Error>> {
    // Start a transaction
    let txn = db.begin().await?;

    // Get the current Utc time
    let now_utc = Utc::now();

    // Convert Utc time to a FixedOffset with a zero offset (UTC)
    let now_fixed = FixedOffset::east_opt(0)
        .ok_or("Failed to create FixedOffset")?
        .from_utc_datetime(&now_utc.naive_utc());

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

    // Insert the first record into the database
    chat_participant_1.insert(&txn).await?;

    // Insert the second record into the database
    chat_participant_2.insert(&txn).await?;

    // Commit the transaction
    txn.commit().await?;

    Ok(())
}
