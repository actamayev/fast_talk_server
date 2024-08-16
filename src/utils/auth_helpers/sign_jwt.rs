use jsonwebtoken::{encode, Header, EncodingKey};
use serde::{Serialize, Deserialize};
use std::error::Error;
use std::env;
use dotenvy::dotenv;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

pub async fn sign_jwt(user_id: &i32) -> Result<String, Box<dyn Error>> {
	dotenv().ok();
    // Fetch your secret key, for example using your SecretsManager equivalent in Rust
    let secret = env::var("JWT_KEY").expect("DATABASE_URL must be set");

    let claims = Claims {
        sub: user_id.to_string(),
        exp: 10000000000, // Set the expiration timestamp
    };

    let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_ref()))?;
    Ok(token)
}
