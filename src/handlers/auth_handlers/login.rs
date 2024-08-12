//src/handlers/login
use actix_web::{web, HttpResponse, Error};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
struct LoginResponse {
    pub access_token: String,
    pub public_key: String,
}

pub async fn login(req: web::Json<LoginRequest>) -> Result<HttpResponse, Error> {
    let access_token = "some_generated_jwt".to_string();
    let public_key = "some_public_key".to_string();

    let response = LoginResponse {
        access_token,
        public_key,
    };

    Ok(HttpResponse::Ok().json(response))
}
