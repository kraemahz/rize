use actix_web::{web, HttpServer, App};
use rize::{login_handler, get_db_pool};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db_pool = get_db_pool().await;
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db_pool.clone()))
            .service(web::resource("/login").route(web::post().to(login_handler)))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
