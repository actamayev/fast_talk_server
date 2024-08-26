use std::error::Error;
use sea_orm::prelude::Expr;
use sea_orm::{ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter, QuerySelect};
use crate::entities::credentials;
use crate::types::globals::{EmailOrUsername, UserInfo};
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

pub async fn find_user_by_username(
    db: &DatabaseConnection,
    username_to_search_for: String,
    username_to_exclude: String
) -> Result<Vec<UserInfo>, DbErr> {
    let pattern = format!("%{}%", username_to_search_for);

    let results = credentials::Entity::find()
        .filter(credentials::Column::Username.like(&pattern)) // Apply the LIKE filter with the pattern
        .filter(credentials::Column::Username.ne(username_to_exclude)) // Exclude the given username
        .select_only()
        .column(credentials::Column::Username)
        .column(credentials::Column::UserId)
        .limit(10)
        .into_tuple::<(String, i32)>() // Directly extract the username and user_id
        .all(db)  // Fetch all matching results
        .await?;

    // Map the results into a vector of UserInfo structs
    let user_infos = results.into_iter()
        .map(|(username, user_id)| UserInfo { username, user_id })
        .collect();

    Ok(user_infos)
}
