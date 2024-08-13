//src/handlers/login
use actix_web::{web, HttpResponse, Error};
use serde::{Deserialize, Serialize};
use validator_derive::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(length(min = 3, max = 100, message = "Contact must be between 3 and 100 characters"))]
    pub contact: String,

    #[validate(length(min = 6, max = 100, message = "Password must be between 6 and 100 characters"))]
    pub password: String,
}

#[derive(Serialize)]
struct LoginResponse {
    pub access_token: String,
    pub public_key: String,
}

pub async fn login(_req: web::Json<LoginRequest>) -> Result<HttpResponse, Error> {
    let access_token = "some_generated_jwt".to_string();
    let public_key = "some_public_key".to_string();

    let response = LoginResponse {
        access_token,
        public_key,
    };

    Ok(HttpResponse::Ok().json(response))
}
