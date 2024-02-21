use actix_web::{App, HttpServer};
use auth::login;
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(login))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}

pub mod auth;
