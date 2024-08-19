use std::env;
use dotenvy::dotenv;
use std::error::Error;
use serde::{Serialize, Deserialize};
use jsonwebtoken::{encode, decode, Header, EncodingKey, DecodingKey, Validation, TokenData};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    exp: usize,
}

pub fn sign_jwt(user_id: &i32) -> Result<String, Box<dyn Error>> {
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

pub fn decode_jwt(access_token: &str) -> Result<Claims, Box<dyn Error>> {
    dotenv().ok();
    // Fetch your secret key
    let secret = env::var("JWT_KEY").expect("JWT_KEY must be set");

    // Validate and decode the token
    let token_data: TokenData<Claims> = decode::<Claims>(
        access_token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    )?;

    // Return the claims inside the token
    Ok(token_data.claims)
}
