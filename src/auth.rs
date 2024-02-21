use actix_web::{web, HttpResponse, Responder};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;
pub mod auth {
    // Placeholder for authentication logic
}

pub struct LoginRequest {
    pub username: String,
    pub password: String,
}
pub async fn login(info: web::Json<LoginRequest>) -> impl Responder {
    // Placeholder for authentication logic
    HttpResponse::Ok().body("Login endpoint reached.")
}
lazy_static! {
    static ref USER_CREDENTIALS: HashMap<String, String> = {
        let mut m = HashMap::new();
        // This is a placeholder for user credentials.
        // In a real-world scenario, you should retrieve these from a database and store passwords securely (hashed and salted).
        m.insert("test_user".to_string(), "test_password".to_string()); // Example user
        m
    };
}
pub fn verify_user_credentials(credentials: &LoginCredentials) -> bool {
    if let Some(stored_password) = USER_CREDENTIALS.get(&credentials.username) {
        return stored_password == &credentials.password;
    }
    false
}
#[derive(Deserialize, Serialize)]
pub struct LoginCredentials {
    // fields as before
}
// all other code from the file remains the same
