use actix_web::{get, post, web, HttpResponse, Responder};

#[get("/user/{user_id}")]
async fn get_user_profile(user_id: web::Path<i32>) -> impl Responder {
    HttpResponse::Ok().json(UserProfile {
        // Example data, needs to be fetched from database
        id: *user_id,
        username: String::from("example_user"),
        email: String::from("user@example.com"),
        preferences: UserPreferences {
            // Example preferences
        }
    })
}

#[post("/user/{user_id}")]
async fn update_user_profile(user_id: web::Path<i32>, profile: web::Json<UserProfile>) -> impl Responder {
    // Implement logic to update user profile, currently just echoing back
    HttpResponse::Ok().json(profile.into_inner())
}

// More API endpoints can be added here
use actix_web::{web, HttpResponse, ResponseError};
use thiserror::Error;

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

// More error conversions would be added here if necessary
#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, web, App};

    #[actix_rt::test]
    async fn test_get_user_profile() {
        let app = test::init_service(App::new().service(get_user_profile)).await;
        let req = test::TestRequest::get().uri("/user/1").to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }

    #[actix_rt::test]
    async fn test_update_user_profile() {
        let profile_data = UserProfile {
            id: 1,
            username: String::from("testuser"),
            email: String::from("test@example.com"),
            preferences: UserPreferences {},
        };
        let app = test::init_service(App::new().service(update_user_profile)).await;
        let req = test::TestRequest::post()
            .uri("/user/1")
            .set_json(&profile_data)
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }

    // More tests can be added here
}
