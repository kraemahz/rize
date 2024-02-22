use actix_web::{web, HttpResponse, Responder};
use bcrypt::verify;
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Deserialize, Serialize)]
pub struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Deserialize, Serialize)]
#[derive(Deserialize, Serialize)]
pub struct LoginResponse {
    message: String,
    token: String,
}

async fn login_handler(
    _db_pool: web::Data<PgPool>,
    login_request: web::Json<LoginRequest>,
) -> impl Responder {
    let _ = login_request.into_inner(); // to avoid unused variable warning
    HttpResponse::Ok().finish()
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

#[derive(Deserialize, Serialize, Debug)]
struct UserProfile {
    id: i32,
    name: String,
    email: String,
    preferences: Preferences,
}
#[derive(Deserialize, Serialize, Debug)]
struct Preferences {
    // Define preferences fields here
    // Example:
    // timezone: String,
    // language: String,
}
async fn get_user_profile(
    _db_pool: web::Data<PgPool>,
    user_id: web::Path<i32>, // to verify the token
) -> impl Responder {
    let _user_id = user_id.into_inner();
    HttpResponse::Ok().finish()
}
async fn update_user_profile(
    _db_pool: web::Data<PgPool>,
    user_id: web::Path<i32>,
    _user_profile: web::Json<UserProfile>,
) -> impl Responder {
    let _user_id = user_id.into_inner();
    HttpResponse::Ok().finish()
}
impl UserProfile {
    fn validate(&self) -> Result<(), String> {
        if self.name.trim().is_empty() {
            return Err("Name cannot be empty".to_string());
        }
        if self.email.trim().is_empty() {
            return Err("Email cannot be empty".to_string());
        }
        if !self.email.contains('@') {
            return Err("Email must be a valid email address".to_string());
        }
        // Add more validations as needed
        Ok(())
    }
}
