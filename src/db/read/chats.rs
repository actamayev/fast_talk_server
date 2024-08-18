use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use std::error::Error;
use crate::entities::chats;

pub async fn does_chat_exist(db: &DatabaseConnection, chat_id: i32) -> Result<bool, Box<dyn Error>> {
    let chat = chats::Entity::find()
        .filter(chats::Column::ChatId.eq(chat_id)) 
        .one(db)
        .await?;

    Ok(chat.is_some())
}