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
    pub contact: String,
    pub password: String,
}

pub async fn login(req: web::Json<LoginRequest>) -> Result<HttpResponse, Error> {
    let contact = req.contact.to_string();
    let password = req.password.to_string();

    let response = LoginResponse {
        contact,
        password,
    };

    Ok(HttpResponse::Ok().json(response))
}
