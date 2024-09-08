use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use serde::{Serialize, Deserialize};
use std::env;
use jsonwebtoken::errors::Result;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,  // User ID or email
    exp: usize,   // Expiration time
}

// Create JWT
pub fn create_jwt(user_id: String) -> String {
    let expiration = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::hours(24))
        .expect("valid timestamp")
        .timestamp();

    let claims = Claims {
        sub: user_id,
        exp: expiration as usize,
    };

    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    ).unwrap()
}

// Decode JWT
pub fn decode_jwt(token: &str) -> Result<Claims> {
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    ).map(|token_data| token_data.claims)
}
