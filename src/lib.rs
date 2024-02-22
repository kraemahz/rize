use actix_web::{App, HttpServer};

mod user_profile;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(user_profile::get_user_profile)
            .service(user_profile::update_user_profile)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
