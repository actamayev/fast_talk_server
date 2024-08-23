use serde_json::json;
use actix_web::{Error, HttpRequest, HttpMessage, HttpResponse};
use crate::types::{globals::AuthenticatedUser, outgoing_responses::PersonalInfoResponse};

pub async fn retrieve_personal_info(req: HttpRequest) -> Result<HttpResponse, Error> {
    let user = match req.extensions().get::<AuthenticatedUser>().cloned() {
        Some(AuthenticatedUser(user)) => user,
        None => {
            return Ok(HttpResponse::Unauthorized().json(json!({"message": "User not found"})));
        }
    };

    let response = PersonalInfoResponse { username: user.username };

    Ok(HttpResponse::Ok().json(response))
}
