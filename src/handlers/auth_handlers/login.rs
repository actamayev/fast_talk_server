use std::time::Instant;
use actix_web::{web, HttpResponse, Error};
use sea_orm::DatabaseConnection;
use crate::utils::auth_helpers::{hash::Hash, jwt::sign_jwt};
use crate::types::{incoming_requests::LoginRequest, outgoing_responses::AuthResponse};
use crate::db::{read::credentials::find_user_by_contact, write::login_history::add_login_history};

pub async fn login(
    db: web::Data<DatabaseConnection>,
    req: web::Json<LoginRequest>
) -> Result<HttpResponse, Error> {
    let start = Instant::now();

    // Timer for finding the user
    let user_lookup_start = Instant::now();
    let user = find_user_by_contact(&db, &req.contact).await?;
    let user_lookup_duration = user_lookup_start.elapsed();
    println!("Time to find user: {:?}", user_lookup_duration);

    let user_match_start = Instant::now();
    let user = match user {
        Some(user) => user,
        None => {
            return Ok(HttpResponse::Unauthorized().json(serde_json::json!({
                "message": "Invalid credentials"
            })));
        }
    };
    let user_match_duration = user_match_start.elapsed();
    println!("Time for user match check: {:?}", user_match_duration);

    // Timer for password check
    let password_check_start = Instant::now();
    let do_passwords_match = Hash::check_password(&req.password, &user.password)
        .map_err(actix_web::error::ErrorInternalServerError)?;
    let password_check_duration = password_check_start.elapsed();
    println!("Time to check password: {:?}", password_check_duration);

    if !do_passwords_match {
        return Ok(HttpResponse::BadRequest().json(serde_json::json!({
            "message": "Wrong password"
        })));
    }

    // Timer for JWT signing
    let jwt_sign_start = Instant::now();
    let access_token = sign_jwt(&user.user_id)?;
    let jwt_sign_duration = jwt_sign_start.elapsed();
    println!("Time to sign JWT: {:?}", jwt_sign_duration);

    // Timer for adding login history
    let login_history_start = Instant::now();
    add_login_history(&db, user.user_id).await?;
    let login_history_duration = login_history_start.elapsed();
    println!("Time to add login history: {:?}", login_history_duration);

    let response = AuthResponse {
        access_token
    };

    let total_duration = start.elapsed();
    println!("Total time for login function: {:?}", total_duration);

    Ok(HttpResponse::Ok().json(response))
}