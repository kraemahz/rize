use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String, // Subject (whom the token refers to)
    exp: usize, // Expiry date
}

pub fn generate_token(username: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let expiration_time = SystemTime::now()
        .checked_add(Duration::new(60 * 60 * 24, 0))
        .expect("valid timestamp")
        .duration_since(UNIX_EPOCH)
        .expect("time went backwards")
        .as_secs() as usize;

    let claims = Claims {
        sub: username.to_owned(),
        exp: expiration_time,
    };

    let header = Header::new(Algorithm::HS256);
    let encoding_key = EncodingKey::from_secret("secret_key".as_ref()); // TODO: use an actual secret key from config or environment

    encode(&header, &claims, &encoding_key)
}
