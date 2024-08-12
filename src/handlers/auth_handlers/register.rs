//src/handlers/register
use actix_web::{web, HttpResponse, Error};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct RegisterResponse {
    pub access_token: String,
    pub public_key: String,
}

pub async fn register(req: web::Json<RegisterRequest>) -> Result<HttpResponse, Error> {
    let access_token = "some_generated_jwt".to_string();
    let public_key = "some_public_key".to_string();

    let response = RegisterResponse {
        access_token,
        public_key,
    };

    Ok(HttpResponse::Ok().json(response))
}
