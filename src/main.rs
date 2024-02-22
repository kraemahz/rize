use actix_web::{web, HttpResponse, Responder, HttpServer, App};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct Credentials {
    username: String,
    password: String,
}

#[derive(Serialize)]
struct LoginResponse {
    message: String,
    token: String, // Placeholder for actual token to be implemented later
}

async fn login(_credentials: web::Json<Credentials>) -> impl Responder {
    let response = LoginResponse {
        message: "Login Successful".to_string(),
        token: "placeholder_token".to_string(),
    };

    HttpResponse::Ok().json(response)
}

pub fn config_services(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/login").route(web::post().to(login)));
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().configure(config_services)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
