use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};
use std::error::Error;
use crate::{entities::credentials, types::globals::CredentialsData};

pub async fn add_credentials_record(db: &DatabaseConnection, credentials_data: CredentialsData) -> Result<i32, Box<dyn Error>> {
    // Create a new ActiveModel instance for the credentials table
    let credentials = credentials::ActiveModel {
        username: Set(credentials_data.username),      // Move the ownership of the string
        password: Set(credentials_data.hashed_password), // Move the ownership of the string
        email: Set(credentials_data.email),            // Move the ownership of the string
        ..Default::default()
    };

    // Insert the new record into the database and get the result
    let insert_result = credentials.insert(db).await?;

    // Return the user ID (primary key)
    Ok(insert_result.user_id)
}
