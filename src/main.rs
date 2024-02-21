use crate::auth::{authenticate_and_respond, LoginCredentials};
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/api/login", web::post().to(login)))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}

extern crate actix_web;
mod auth;
async fn login(credentials: web::Json<LoginCredentials>) -> impl Responder {
    authenticate_and_respond(credentials).await
}
#[cfg(test)]
mod tests {
    use crate::auth::{authenticate_and_respond, LoginCredentials};
    use actix_web::{http::StatusCode, test, App};

    #[actix_rt::test]
    async fn test_login_success() {
        let mut app = test::init_service(
            App::new().route("/api/login", web::post().to(authenticate_and_respond)),
        )
        .await;
        let credentials = LoginCredentials {
            username: "test_user".to_string(),
            password: "test_password".to_string(),
        };
        let req = test::TestRequest::post()
            .uri("/api/login")
            .set_json(&credentials)
            .to_request();
        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn test_login_failure() {
        let mut app = test::init_service(
            App::new().route("/api/login", web::post().to(authenticate_and_respond)),
        )
        .await;
        let credentials = LoginCredentials {
            username: "wrong_user".to_string(),
            password: "wrong_password".to_string(),
        };
        let req = test::TestRequest::post()
            .uri("/api/login")
            .set_json(&credentials)
            .to_request();
        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
    }
}
