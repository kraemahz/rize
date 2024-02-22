#[cfg(test)]
mod api_tests {
    use actix_web::{test, web, App, http::{self, StatusCode}, Error};
    use sqlx::{PgPool, Pool, Postgres};
    use crate::{login_handler, get_user_profile, update_user_profile};

    // Mock user profile for testing
    fn mock_user_profile() -> crate::UserProfile {
        crate::UserProfile {
            id: 1,
            name: "Jane Doe".to_string(),
            email: "jane.doe@example.com".to_string(),
            preferences: crate::Preferences { /* populate with test data */ },
        }
    }

    #[actix_rt::test]
    async fn test_get_user_profile() -> Result<(), Error> {
        let pool: Pool<Postgres> = setup_db().await.unwrap();
        let data_pool = web::Data::new(pool);
        let app = test::init_service(App::new().app_data(data_pool.clone()).service(web::resource("/users/{user_id}").route(web::get().to(get_user_profile)))).await;

        let req = test::TestRequest::get().uri("/users/1").to_request();
        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), StatusCode::OK);

        // More assertions for verifying the response

        Ok(())
    }

    #[actix_rt::test]
    async fn test_update_user_profile() -> Result<(), Error> {
        let pool: Pool<Postgres> = setup_db().await.unwrap();
        let data_pool = web::Data::new(pool);
        let app = test::init_service(App::new().app_data(data_pool.clone()).service(web::resource("/users/{user_id}").route(web::put().to(update_user_profile)))).await;

        let user_profile = mock_user_profile();
        let req = test::TestRequest::put().uri("/users/1").set_json(&user_profile).to_request();
        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), StatusCode::OK);

        // More assertions for verifying the response

        Ok(())
    }

    // Additional helper functions for DB setup and teardown
}