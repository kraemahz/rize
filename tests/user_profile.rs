#[cfg(test)]
mod tests {
    use crate::user_profile::*;
    use actix_web::{http::StatusCode, test, web, App};
    use sqlx::{Executor, PgPool};

    // Set up the test database connection
    async fn setup_test_db() -> PgPool {
        let database_url =
            std::env::var("DATABASE_URL_TEST").expect("DATABASE_URL_TEST must be set");
        PgPool::connect(&database_url).await.unwrap()
    }

    #[actix_rt::test]
    async fn test_create_user_profile() {
        let pool = setup_test_db().await;
        let mut app = test::init_service(
            App::new()
                .data(pool.clone())
                .route("/user_profile", web::post().to(create_user_profile)),
        )
        .await;

        // Create a new profile request
        let new_profile = web::Json(UserProfile {
            id: 0, // id is auto-generated
            username: "testuser".to_string(),
            email: "testuser@example.com".to_string(),
            first_name: Some("Test".to_string()),
            last_name: Some("User".to_string()),
            avatar_url: Some("http://example.com/avatar.png".to_string()),
        });

        let req = test::TestRequest::post()
            .uri("/user_profile")
            .set_json(&new_profile)
            .to_request();

        let resp = test::call_service(&mut app, req).await;

        // Check for success response
        assert_eq!(resp.status(), StatusCode::CREATED);

        // Verify that the record was inserted into the database
        let result: UserProfile = sqlx::query_as!(
            UserProfile,
            "SELECT * FROM user_profiles WHERE username = $1",
            "testuser"
        )
        .fetch_one(&pool)
        .await
        .unwrap();

        assert_eq!(result.username, new_profile.username);
        assert_eq!(result.email, new_profile.email);
        assert_eq!(result.first_name, new_profile.first_name);
        assert_eq!(result.last_name, new_profile.last_name);
        assert_eq!(result.avatar_url, new_profile.avatar_url);
    }

    // Additional tests for read, update, and delete will follow a similar structure
}
