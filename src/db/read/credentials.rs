use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use std::error::Error;

pub async fn does_email_exist(db: &DatabaseConnection, email: &str) -> Result<bool, Box<dyn Error>> {
    use crate::entities::credentials;

    let user = credentials::Entity::find()
    // TODO: Change this to ilike for case-insentisitve searches
        .filter(credentials::Column::Email.like(email)) // Use ilike with PostgreSQL extension
        .one(db)
        .await?;

    Ok(user.is_some())
}

pub async fn does_username_exist(db: &DatabaseConnection, username: &str) -> Result<bool, Box<dyn Error>> {
    use crate::entities::credentials;

    let user = credentials::Entity::find()
    // TODO: Change this to ilike for case-insentisitve searches
        .filter(credentials::Column::Username.like(username)) // Use ilike with PostgreSQL extension
        .one(db)
        .await?;

    Ok(user.is_some())
}

