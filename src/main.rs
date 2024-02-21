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
