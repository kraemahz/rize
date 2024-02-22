use crate::server::{get_user_profile, login_handler, update_user_profile};
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use rize::login_handler;
use sqlx::PgPool;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db_pool = PgPool::connect_lazy(&database_url).unwrap();
    let data_pool = web::Data::new(db_pool);

    HttpServer::new(move || {
        App::new()
            .app_data(data_pool.clone())
            .route("/login", web::post().to(login_handler))
            .route("/users/{user_id}", web::get().to(get_user_profile))
            .route("/users/{user_id}", web::put().to(update_user_profile))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
