use serde::Serialize;
use chrono::{DateTime, FixedOffset};

#[derive(Serialize)]
pub struct AuthResponse {
    pub access_token: String,
    pub username: String
}

#[derive(Serialize)]
pub struct PersonalInfoResponse {
    pub username: String
}

#[derive(Serialize)]
pub struct CreateChatResponse {
    pub chat_id: i32
}

#[derive(Serialize)]
pub struct SendMessageResponse {
    pub message_id: i32
}

#[derive(Serialize)]
pub struct SingleRetrievedChat {
    pub chat_id: i32,
    pub friend_username: String,
    pub friend_user_id: i32,
    pub last_message: String,
    pub last_message_time: DateTime<FixedOffset>,
    pub was_last_message_sent_by_user: bool,
    pub chat_created_at: DateTime<FixedOffset>
}

#[derive(Serialize)]
pub struct ChatMessage {
    pub message_id: i32,
    pub did_user_send: bool,
    pub sender_user_id: i32,
    pub message_text: String,
    pub sent_time: DateTime<FixedOffset>
}

#[derive(Serialize)]
pub struct OutgoingSocketMessage {
    pub chat_id: i32,
    pub message_id: i32,
    pub message_text: String,
    pub sent_time: DateTime<FixedOffset>,
    pub message_sender_user_id: i32,
    pub message_sender_username: String
}
