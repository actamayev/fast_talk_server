use sea_orm::prelude::Expr;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use std::error::Error;
use crate::entities::credentials;
use crate::types::globals::EmailOrUsername;
use crate::utils::auth_helpers::determine_contact_type::determine_login_contact_type;

pub async fn does_email_exist(db: &DatabaseConnection, email: &str) -> Result<bool, Box<dyn Error>> {
    let user = credentials::Entity::find()
        .filter(Expr::col(credentials::Column::Email).eq(Expr::val(email.to_lowercase()))) 
        .one(db)
        .await?;

    Ok(user.is_some())
}

pub async fn does_username_exist(db: &DatabaseConnection, username: &str) -> Result<bool, Box<dyn Error>> {
    let user = credentials::Entity::find()
        .filter(Expr::col(credentials::Column::Username).eq(Expr::val(username.to_lowercase()))) 
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
                .filter(Expr::col(credentials::Column::Email).eq(Expr::val(contact.to_lowercase()))) 
                .one(db)
                .await?
        }
        EmailOrUsername::Username => {
            credentials::Entity::find()
                .filter(Expr::col(credentials::Column::Username).eq(Expr::val(contact.to_lowercase()))) 
                .one(db)
                .await?
        }
    };

    Ok(user)
}

pub async fn find_user_by_id(
    db: &DatabaseConnection,
    user_id: i32,
) -> Result<Option<credentials::Model>, Box<dyn Error>> {
    let user = credentials::Entity::find()
        .filter(credentials::Column::UserId.eq(user_id))
        .one(db)
        .await?;

    Ok(user)
}
