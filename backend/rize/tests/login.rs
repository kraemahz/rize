use actix_web::{http, test, web, App};
use rize::login;
use rize::LoginRequest;

#[actix_rt::test]
async fn test_successful_login() {
    let mut app = test::init_service(App::new().route("/login", web::post().to(login))).await;

    let login_req = LoginRequest {
        username: "example_user".to_string(),
        password: "example_password".to_string(),
    };

    let req = test::TestRequest::post()
        .uri("/login")
        .set_json(&login_req)
        .to_request();

    let resp = test::call_service(&mut app, req).await;
    assert_eq!(resp.status(), http::StatusCode::OK);

    let response_body: ApiResponse = test::read_body_json(resp).await;
    assert_eq!(
        response_body.message,
        "Welcome, example_user! You are now logged in."
    );
}

#[actix_rt::test]
async fn test_failed_login_incorrect_password() {
    let mut app = test::init_service(App::new().route("/login", web::post().to(login))).await;

    let login_req = LoginRequest {
        username: "example_user".to_string(),
        password: "wrong_password".to_string(),
    };

    let req = test::TestRequest::post()
        .uri("/login")
        .set_json(&login_req)
        .to_request();

    let resp = test::call_service(&mut app, req).await;
    assert_eq!(resp.status(), http::StatusCode::UNAUTHORIZED);

    let response_body: String = test::read_body(resp).await;
    assert_eq!(response_body, "Invalid username or password.");
}
