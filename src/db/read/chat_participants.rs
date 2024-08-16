use sea_orm::entity::prelude::*;
use sea_orm::{DatabaseConnection, DbErr, QuerySelect};
use crate::entities::chat_participants;

pub async fn does_existing_chat_exist(
    db: &DatabaseConnection,
    user_id1: i32,
    user_id2: i32,
) -> Result<bool, DbErr> {

    // Query for chats where user_id1 is a participant
    let user1_chats = chat_participants::Entity::find()
        .filter(chat_participants::Column::UserId.eq(user_id1))
        .select_only()
        .column(chat_participants::Column::ChatId)
        .into_tuple::<i32>()
        .all(db)
        .await?;

    // Query for chats where user_id2 is a participant
    let user2_chats = chat_participants::Entity::find()
        .filter(chat_participants::Column::UserId.eq(user_id2))
        .select_only()
        .column(chat_participants::Column::ChatId)
        .into_tuple::<i32>()
        .all(db)
        .await?;

    // Check if there's a common chat_id between user1 and user2
    let common_chat_exists = user1_chats.iter().any(|chat_id1| user2_chats.contains(chat_id1));

    Ok(common_chat_exists)
}
