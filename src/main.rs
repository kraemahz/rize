use actix_web_httpauth::middleware::HttpAuthentication;
use actix_web_httpauth::extractors::basic::{BasicAuth, Config};
use actix_web::dev::ServiceRequest;
use std::sync::Mutex;
use serde::{Deserialize, Serialize};
use auth::{authenticate_and_respond, LoginCredentials};
use crate::auth::LoginCredentials;
use crate::auth::{authenticate_and_respond, LoginCredentials};
use actix_web::{web, App, HttpServer, middleware, HttpResponse};
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let auth = HttpAuthentication::basic(validator);
    println!("Server running at http://localhost:8080");
    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(auth)
            .route("/login", web::post().to(login))
    })
    .bind("localhost:8080")?
    .run()
    .await
}

extern crate actix_web;
mod auth;
async fn login(info: web::Json<LoginInfo>) -> HttpResponse {
    if info.username == "admin" && info.password == "password" {
        HttpResponse::Ok().json(LoginResponse { message: "Logged in successfully" })
    } else {
        HttpResponse::BadRequest().json(LoginResponse { message: "Error logging in" })
    }
}
#[cfg(test)]
mod tests {
    use actix_web::{test, web, http::StatusCode};
    use crate::auth::{LoginCredentials, authenticate_and_respond};

    #[actix_rt::test]
    async fn test_login_success() {
        let mut app = test::init_service(App::new().route("/api/login", web::post().to(login_handler))).await;
        let credentials = web::Json(LoginCredentials {
            username: "User1".to_string(),
            password: "Password1".to_string(),
        });
        let req = test::TestRequest::post().uri("/api/login").set_json(&credentials).to_request();
        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn test_login_failure() {
        let mut app = test::init_service(App::new().route("/api/login", web::post().to(login_handler))).await;
        let credentials = web::Json(LoginCredentials {
            username: "User2".to_string(),
            password: "WrongPassword".to_string(),
        });
        let req = test::TestRequest::post().uri("/api/login").set_json(&credentials).to_request();
        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
    }
}
// ... rest of the file remains unchanged
lazy_static::lazy_static! {
    static ref USER_DATA: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
}
#[derive(Deserialize)]
struct LoginRequest {
    username: String,
    password: String,
}
#[derive(Serialize)]
struct LoginResponse {
    message: String,
}
// Login Handler
async fn validator(req: ServiceRequest, credentials: BasicAuth) -> Result<ServiceRequest, actix_web::Error> {
    let config = req.app_data::<Config>().cloned().unwrap_or_default();
    // Here we handle the provided credentials, for example:
    match credentials.user_id().as_ref() {
        "user" if credentials.password().unwrap() == "pass" => Ok(req),
        _ => Err(actix_web::error::ErrorUnauthorized("Invalid username or password")),
    }
}
#[derive(Deserialize)]
struct LoginInfo {
    username: String,
    password: String,
}