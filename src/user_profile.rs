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
