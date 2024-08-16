use serde::Serialize;

#[derive(Serialize)]
pub struct AuthResponse {
    pub access_token: String
}
