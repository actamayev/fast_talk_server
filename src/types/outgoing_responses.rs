use serde::Serialize;

#[derive(Serialize)]
pub struct AuthResponse {
    pub access_token: String
}

#[derive(Serialize)]
pub struct CreateChatResponse {
    pub chat_id: i32
}

#[derive(Serialize)]
pub struct SendMessageResponse {
    pub message_id: i32
}
