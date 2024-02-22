#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, web, App};
    use sqlx::{Pool, Postgres};

    #[actix_rt::test]
    async fn test_update_user_success() {
        let mut app = test::init_service(
            App::new().route("/users/{user_id}", web::put().to(update_user_handler)),
        )
        .await;

        let req = test::TestRequest::put()
            .uri("/users/123")
            .set_json(&UpdateUserRequest {
                username: "testuser".to_string(),
                email: Some("test@example.com".to_string()),
                full_name: Some("Test User".to_string()),
                preferences: None,
            })
            .to_request();

        let resp = test::call_service(&mut app, req).await;

        assert!(resp.status().is_success());
    }

    #[actix_rt::test]
    async fn test_update_user_failure() {
        let mut app = test::init_service(
            App::new().route("/users/{user_id}", web::put().to(update_user_handler)),
        )
        .await;

        // Simulate a failure by sending invalid data, e.g., missing username
        let req = test::TestRequest::put()
            .uri("/users/123")
            .set_json(&UpdateUserRequest {
                username: "".to_string(),
                email: None,
                full_name: None,
                preferences: None,
            })
            .to_request();

        let resp = test::call_service(&mut app, req).await;

        assert!(resp.status().is_client_error());
    }
}
use actix_web::http::StatusCode;
use actix_web::{web, HttpResponse, Responder};
use bcrypt::verify;
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::PgPool;
use sqlx::{Executor, PostgreSQL};
use std::env;

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

#[derive(Deserialize, Serialize)]
pub struct UpdateUserRequest {
    pub username: String,
    pub email: Option<String>,
    pub full_name: Option<String>,
    pub preferences: Option<serde_json::Value>, // A JSON field to store custom user preferences
}

#[derive(Serialize)]
pub struct UpdateUserResponse {
    pub message: String,
}

// The function will need to be fleshed out with logic to handle the update.
pub async fn update_user_handler(
    db_pool: web::Data<PgPool>,
    user_id: web::Path<i32>,
    user_update_request: web::Json<UpdateUserRequest>,
) -> impl Responder {
    match execute_update_user(&db_pool, *user_id, &user_update_request).await {
        Ok(_) => HttpResponse::Ok().json(UpdateUserResponse {
            message: "User updated successfully".to_string(),
        }),
        Err(e) => {
            HttpResponse::build(StatusCode::BAD_REQUEST).json(UpdateUserResponse { message: e })
        }
    }
}
use crate::update_user_handler;
async fn update_user_profile(
    db_pool: &web::Data<PgPool>,
    user_id: i32, // Assuming user_id is an i32
    update_request: &UpdateUserRequest,
) -> Result<(), String> {
    // Validation logic will go here

    // Database update logic will go here

    Ok(())
}
pub async fn execute_update_user(
    db_pool: &web::Data<PgPool>,
    user_id: i32, // Assuming user_id is an i32
    update_request: &UpdateUserRequest,
) -> Result<(), String> {
    // Example of a SQLx transaction
    let mut tx = db_pool.begin().await.map_err(|e| e.to_string())?;

    // Validation logic will go here
    if update_request.email.is_none()
        && update_request.full_name.is_none()
        && update_request.preferences.is_none()
    {
        return Err("No update information provided".to_string());
    }

    // Constructing the update SQL query
    // You must adapt this query to match your actual user table structure and column names
    let mut query = "UPDATE users SET".to_string();

    if let Some(email) = &update_request.email {
        query = format!("{} email = $1,", query);
    }
    if let Some(full_name) = &update_request.full_name {
        query = format!("{} full_name = $2,", query);
    }
    if let Some(preferences) = &update_request.preferences {
        query = format!("{} preferences = $3,", query);
    }

    query.pop(); // Remove the trailing comma
    query = format!("{} WHERE id = $4", query);

    // Execute the update
    tx.execute(
        &query,
        &[
            &update_request.email.unwrap_or_default(),
            &update_request.full_name.unwrap_or_default(),
            &json!(update_request.preferences),
            &user_id,
        ],
    )
    .await
    .map_err(|e| e.to_string())?;

    tx.commit().await.map_err(|e| e.to_string())?;

    Ok(())
}
