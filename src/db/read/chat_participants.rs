use sea_orm::entity::prelude::*;
use sea_orm::{DatabaseConnection, DbErr, QuerySelect};
use crate::entities::chat_participants;

pub async fn does_existing_chat_exist(
    db: &DatabaseConnection,
    user_id1: i32,
    user_id2: i32,
) -> Result<Option<i32>, DbErr> {
    // Query for chat IDs where user_id1 is a participant
    let user1_chats = chat_participants::Entity::find()
        .filter(chat_participants::Column::UserId.eq(user_id1))
        .select_only()
        .column(chat_participants::Column::ChatId)
        .into_tuple::<i32>()
        .all(db)
        .await?;

    // Query for chat IDs where user_id2 is a participant
    let user2_chats = chat_participants::Entity::find()
        .filter(chat_participants::Column::UserId.eq(user_id2))
        .select_only()
        .column(chat_participants::Column::ChatId)
        .into_tuple::<i32>()
        .all(db)
        .await?;

    // Find the first common chat_id between user1 and user2
    for chat_id1 in user1_chats {
        if user2_chats.contains(&chat_id1) {
            return Ok(Some(chat_id1)); // Return the common chat_id
        }
    }

    // If no common chat_id is found, return None
    Ok(None)
}

pub async fn is_user_in_chat(
    db: &DatabaseConnection,
    user_id: i32,
    chat_id: i32,
) -> Result<bool, DbErr> {
    // Query the chat participants table for a record that matches both user_id and chat_id
    let participant_exists = chat_participants::Entity::find()
        .filter(chat_participants::Column::UserId.eq(user_id))
        .filter(chat_participants::Column::ChatId.eq(chat_id))
        .one(db)
        .await?
        .is_some(); // Check if a matching record exists

    Ok(participant_exists)
}

pub async fn get_user_chat_ids(
    db: &DatabaseConnection,
    user_id: i32,
) -> Result<Vec<i32>, DbErr> {
    // Query the chat participants table for all chat_id's associated with the user_id
    let chat_ids = chat_participants::Entity::find()
        .filter(chat_participants::Column::UserId.eq(user_id))
        .select_only()
        .column(chat_participants::Column::ChatId)
        .into_tuple::<i32>()
        .all(db)
        .await?;

    Ok(chat_ids) // Return the vector of chat_id's
}

pub async fn get_other_user_in_chat(
    db: &DatabaseConnection,
    chat_id: i32,
    user_id: i32,
) -> Result<Option<i32>, DbErr> {
    // Query the chat participants table for the other participant's user_id
    let other_user = chat_participants::Entity::find()
        .filter(chat_participants::Column::ChatId.eq(chat_id))  // Filter by chat_id
        .filter(chat_participants::Column::UserId.ne(user_id))  // Exclude the passed user_id
        .select_only()
        .column(chat_participants::Column::UserId)  // Select only the user_id column
        .into_tuple::<i32>()  // Map the result to a tuple of i32 (the user_id)
        .one(db)
        .await?;  // Fetch the result, returning an Option<i32>

    Ok(other_user)
}
