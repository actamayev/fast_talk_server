use serde::Serialize;

#[derive(Serialize)]
pub struct AuthResponse {
    pub access_token: String
}

#[derive(Serialize)]
pub struct CreateChatResponse {
    pub chat_id: i32
}

