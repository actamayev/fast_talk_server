use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use std::error::Error;
use crate::entities::credentials;
use crate::types::types::EmailOrUsername;
use crate::utils::auth_helpers::determine_contact_type::determine_login_contact_type;

pub async fn does_email_exist(db: &DatabaseConnection, email: &str) -> Result<bool, Box<dyn Error>> {
    let user = credentials::Entity::find()
    // TODO: Change this to ilike for case-insentisitve searches
        .filter(credentials::Column::Email.like(email)) 
        .one(db)
        .await?;

    Ok(user.is_some())
}

pub async fn does_username_exist(db: &DatabaseConnection, username: &str) -> Result<bool, Box<dyn Error>> {
    let user = credentials::Entity::find()
    // TODO: Change this to ilike for case-insentisitve searches
        .filter(credentials::Column::Username.like(username))
        .one(db)
        .await?;

    Ok(user.is_some())
}

pub async fn find_user_by_contact(
    db: &DatabaseConnection,
    contact: &str,
) -> Result<Option<credentials::Model>, Box<dyn Error>> {
    let contact_type = determine_login_contact_type(contact);

    let user = match contact_type {
        EmailOrUsername::Email => {
            credentials::Entity::find()
                .filter(credentials::Column::Email.like(contact))
                .one(db)
                .await?
        }
        EmailOrUsername::Username => {
            credentials::Entity::find()
                .filter(credentials::Column::Username.like(contact))
                .one(db)
                .await?
        }
    };

    Ok(user)
}