pub use user_profile::*;
mod user_profile;
use actix_web::{web, HttpResponse, Responder};
use bcrypt::verify;
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Deserialize, Serialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub message: String,
    pub token: String,
}

pub async fn login_handler(
    _db_pool: web::Data<PgPool>, // to be used when integrating with database
    login_request: web::Json<LoginRequest>,
) -> impl Responder {
    match authenticate(&login_request.username, &login_request.password).await {
        Ok(token) => HttpResponse::Ok().json(LoginResponse {
            message: "Login successful".to_string(),
            token,
        }),
        Err(_) => HttpResponse::Unauthorized().body("Invalid credentials".to_string()),
    }
}

pub async fn authenticate(
    _username: &str, // to be used when integrating with database
    password: &str,
) -> Result<String, String> {
    // Here you would fetch the user data from the database and verify the password
    // This is a placeholder implementation
    let password_hash = "$2b$12$exSx.uDIsxJ5quCH9QB8UuBhIqyIqiSOF/cUraRNSq5ERTlG60eJm"; // Example hash
    match verify(password, password_hash) {
        Ok(valid) => {
            if valid {
                Ok("placeholder_token".to_string()) // Placeholder token generation
            } else {
                Err("Invalid credentials".to_string())
            }
        }
        Err(_) => Err("Invalid credentials".to_string()),
    }
}
