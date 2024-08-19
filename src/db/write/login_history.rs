use std::error::Error;
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};
use crate::entities::login_history;

pub async fn add_login_history(db: &DatabaseConnection, user_id: i32) -> Result<(), Box<dyn Error>> {
    // Create a new ActiveModel instance for the login_history table
    let login_history = login_history::ActiveModel {
        user_id: Set(user_id),
        ..Default::default() // Default other fields (login_history_id will be auto-incremented)
    };

    // Insert the new record into the database
    login_history.insert(db).await?;

    Ok(())
}
