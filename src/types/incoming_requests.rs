use serde::{Deserialize, Serialize};
use validator_derive::Validate;

#[derive(Debug, Deserialize, Validate, Serialize)] // Derive Serialize
#[serde(deny_unknown_fields)]  // This attribute will deny any unknown fields in the JSON
pub struct RegisterRequest {
    #[validate(email(message = "Invalid email format"))]
    pub email: String,

    #[validate(length(min = 3, max = 100, message = "Username must be between 3 and 100 characters"))]
    pub username: String,

    #[validate(length(min = 6, max = 100, message = "Password must be between 6 and 100 characters"))]
    pub password: String,
}

#[derive(Debug, Deserialize, Validate, Serialize)] // Derive Serialize
#[serde(deny_unknown_fields)]  // This attribute will deny any unknown fields in the JSON
pub struct LoginRequest {
    #[validate(length(min = 3, max = 100, message = "Contact must be between 3 and 100 characters"))]
    pub contact: String,

    #[validate(length(min = 6, max = 100, message = "Password must be between 6 and 100 characters"))]
    pub password: String,
}

#[derive(Debug, Deserialize, Validate, Serialize)] // Derive Serialize
#[serde(deny_unknown_fields)]  // This attribute will deny any unknown fields in the JSON
pub struct NewMessageRequest {
    #[validate(length(max = 1000, message = "Message can't be more than 100 characters"))]
    pub message: String,
}
