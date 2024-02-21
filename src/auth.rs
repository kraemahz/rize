use actix_web::{post, web, HttpResponse};
use bcrypt::{hash, verify};
use serde::{Deserialize, Serialize};
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize)]
struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Serialize)]
struct LoginResponse {
    message: String,
    user_info: Option<UserInfo>, // Placeholder for user information that would be returned on successful login
}

#[derive(Serialize)]
struct UserInfo {
    user_id: i32,
    username: String,
    email: String,
}

// Placeholder function for validating user credentials. To be implemented.
fn validate_credentials(login_request: &LoginRequest) -> bool {
    // This will involve querying the database and comparing hashed passwords.
    true // Placeholder return value
}

#[post("/login")]
async fn login(info: web::Json<LoginRequest>) -> HttpResponse {
    if validate_credentials(&info.0) {
        HttpResponse::Ok().json(LoginResponse {
            message: "Login successful".to_string(),
            user_info: Some(UserInfo {
                // Placeholder data
                user_id: 1,
                username: info.username.clone(),
                email: "user@example.com".to_string(),
            }),
        })
    } else {
        HttpResponse::Unauthorized().json(LoginResponse {
            message: "Invalid credentials".to_string(),
            user_info: None,
        })
    }
}

// Placeholder `cfg` attribute to conditionally compile the auth module for tests
#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, web, App};

    #[actix_web::test]
    async fn test_login_success() {
        let req = test::TestRequest::post()
            .set_json(&LoginRequest {
                username: "test_user".to_string(),
                password: "test_password".to_string(),
            })
            .to_request();
        let resp = test::call_service(&mut App::new().service(login), req).await;
        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    async fn test_login_failure() {
        let req = test::TestRequest::post()
            .set_json(&LoginRequest {
                username: "wrong_user".to_string(),
                password: "wrong_password".to_string(),
            })
            .to_request();
        let resp = test::call_service(&mut App::new().service(login), req).await;
        assert!(resp.status().is_client_error());
    }
}
