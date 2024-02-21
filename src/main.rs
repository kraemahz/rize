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
async fn login(info: web::Json<auth::LoginRequest>) -> impl Responder {
    HttpResponse::Ok().body("Login endpoint reached.")
}
