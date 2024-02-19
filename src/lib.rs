use bcrypt::{hash, verify};
use jsonwebtoken::{encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// This struct represents a user in the system.
#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub hashed_password: String,
}
impl User {
    /// Creates a new user with a given username and password
    /// The password is hashed using bcrypt before storing.
    pub fn new(username: String, password: String) -> Self {
        let hashed_password = hash(password, bcrypt::DEFAULT_COST).unwrap();
        User {
            id: Uuid::new_v4(),
            username,
            hashed_password,
        }
    }

    /// Verifies the password against the stored hash.
    pub fn verify_password(&self, password: &str) -> bool {
        verify(password, &self.hashed_password).unwrap()
    }
}
/// Represents the claims to be encoded in the JWT.
#[derive(Serialize, Deserialize)]
struct Claims {
    sub: String, // Subject (whom the token refers to)
    exp: usize,  // Expiry
}
/// Generates a JWT for a given username and expiry period.
fn generate_jwt(username: String, expiry_period_in_hours: i64) -> String {
    let expiration = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::hours(expiry_period_in_hours))
        .expect("Invalid Timestamp")
        .timestamp();

    let claims = Claims {
        sub: username,
        exp: expiration as usize,
    };

    let header = Header::default();
    encode(
        &header,
        &claims,
        &EncodingKey::from_secret("secret_key".as_ref()),
    )
    .unwrap()
}
/// Verifies the JWT token and returns the username if valid.
pub fn verify_jwt(token: &str) -> Option<String> {
    let validation = Validation {
        ..Default::default()
    };
    jsonwebtoken::decode::<Claims>(
        token,
        &DecodingKey::from_secret("secret_key".as_ref()),
        &validation,
    )
    .map(|data| data.claims.sub)
    .ok()
}
