use actix_web::{web, get, post, HttpResponse, Responder, ResponseError};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct UserProfile {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub preferences: UserPreferences,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserPreferences {
    // Define the fields related to user preferences here
}

#[get("/user/{_user_id}")]
async fn get_user_profile(_user_id: web::Path<i32>) -> impl Responder {
    HttpResponse::Ok().json(UserProfile {
        // Example data, needs to be fetched from database
        id: 1, // placeholder value
        username: String::from("example_user"),
        email: String::from("user@example.com"),
        preferences: UserPreferences {
            // Example preferences
        }
    })
}

#[post("/user/{_user_id}")]
async fn update_user_profile(_user_id: web::Path<i32>, profile: web::Json<UserProfile>) -> impl Responder {
    HttpResponse::Ok().json(profile.into_inner())
}

#[allow(dead_code)]
#[derive(Error, Debug)]
pub enum UserProfileError {
    #[error("user not found")]
    NotFound,
    #[error("invalid data provided")]
    InvalidData,
    #[error("internal server error")]
    InternalServerError,
}

impl ResponseError for UserProfileError {
    fn error_response(&self) -> HttpResponse {
        match *self {
            UserProfileError::NotFound => HttpResponse::NotFound().finish(),
            UserProfileError::InvalidData => HttpResponse::BadRequest().finish(),
            UserProfileError::InternalServerError => HttpResponse::InternalServerError().finish(),
        }
    }
}

// More API endpoints and error conversions would be added here
