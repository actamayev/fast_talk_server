use serde_json::json;
use sea_orm::DatabaseConnection;
use actix_web::{web, Error, HttpRequest, HttpMessage, HttpResponse};
use crate::{db::read::credentials::find_user_by_username, types::globals::AuthenticatedUser};

pub async fn search_for_usernames(
    db: web::Data<DatabaseConnection>,
	req: HttpRequest,
	path: web::Path<String>, // Extract chatId from the path
) -> Result<HttpResponse, Error> {
	let username = path.into_inner();

	let user = match req.extensions().get::<AuthenticatedUser>().cloned() {
        Some(AuthenticatedUser(user)) => user,
        None => {
            return Ok(HttpResponse::Unauthorized().json(json!({"message": "User not found"})));
        }
    };

    // Perform the username search and return the appropriate HTTP response
    let response = match find_user_by_username(&db, username, user.username).await {
        Ok(user_infos) => HttpResponse::Ok().json(json!({
            "usernames": user_infos.into_iter().map(|user_info| {
                json!({
                    "username": user_info.username,
                    "user_id": user_info.user_id
                })
            }).collect::<Vec<_>>()
        })),
        Err(_) => HttpResponse::InternalServerError().json(json!({"message": "Database error"})),
    };

    Ok(response)
}
