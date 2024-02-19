use uuid::Uuid;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().configure(configure)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

async fn login(credentials: web::Json<Credentials>) -> impl Responder {
    HttpResponse::Ok().body("Login endpoint hit successfully")
}
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/login").route(web::post().to(login)),
    );
}
#[derive(serde::Deserialize)]
struct Credentials {
    username: String,
    password: String,
}
#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, web, App, http::StatusCode};

    #[actix_rt::test]
    async fn test_login_success() {
        let mut app = test::init_service(App::new().configure(configure)).await;
        let req = test::TestRequest::post()
            .uri("/login")
            .set_json(&Credentials {
                username: "example_user".to_string(),
                password: "example_password".to_string(),
            })
            .to_request();
        let resp = test::call_service(&mut app, req).await;
        assert!(resp.status().is_success());
    }

    #[actix_rt::test]
    async fn test_login_failure() {
        let mut app = test::init_service(App::new().configure(configure)).await;
        let req = test::TestRequest::post()
            .uri("/login")
            .set_json(&Credentials {
                username: "example_user".to_string(),
                password: "wrong_password".to_string(),
            })
            .to_request();
        let resp = test::call_service(&mut app, req).await;
        assert!(resp.status().is_client_error());
    }

    #[actix_rt::test]
    async fn test_login_endpoint() {
        let mut app = test::init_service(App::new().configure(configure)).await;
        let req = test::TestRequest::with_header("content-type", "application/json")
            .uri("/login")
            .set_json(&Credentials {
                username: "example_user".to_string(),
                password: "example_password".to_string()
            })
            .to_request();

        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }
}