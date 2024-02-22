use crate::configure_routes;
#[cfg(test)]
mod tests {
    use super::*;

    use actix_web::{http::StatusCode, test, web, App};
    use serde_json::json;

    #[actix_rt::test]
    async fn test_successful_login() {
        let app = test::init_service(App::new().configure(configure_routes)).await;
        let credentials = json!({
            "username": "test_user",
            "password": "correct_password"
        });
        let req = test::TestRequest::post()
            .uri("/login")
            .set_json(&credentials)
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn test_failed_login() {
        let app = test::init_service(App::new().configure(configure_routes)).await;
        let credentials = json!({
            "username": "test_user",
            "password": "wrong_password"
        });
        let req = test::TestRequest::post()
            .uri("/login")
            .set_json(&credentials)
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
    }
}
