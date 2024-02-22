use crate::user_profile::{
    create_user_profile, delete_user_profile, read_user_profile, update_user_profile,
};
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use rize::login_handler;
use sqlx::PgPool;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // ... existing code ...
    HttpServer::new(move || {
        App::new()
            .app_data(data_pool.clone())
            .service(web::resource("/user_profile").route(web::post().to(create_user_profile)))
            .service(web::resource("/user_profile/{id}").route(web::get().to(read_user_profile)))
            .service(web::resource("/user_profile/{id}").route(web::put().to(update_user_profile)))
            .service(
                web::resource("/user_profile/{id}").route(web::delete().to(delete_user_profile)),
            )
    })
    // ... existing code ...
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db_pool = PgPool::connect_lazy(&database_url).unwrap();
    let data_pool = web::Data::new(db_pool);

    HttpServer::new(move || {
        App::new()
            .app_data(data_pool.clone())
            .service(web::resource("/login").route(web::post().to(login_handler)))
            // Add user profile endpoints here...
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}