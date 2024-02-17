use crate::auth::generate_token;
use crate::db::validate_credentials;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use sqlx::PgPool;

async fn index() -> impl Responder {
    "Welcome to the Rize API!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/", web::get().to(index)))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
async fn login(pool: web::Data<PgPool>, form: web::Json<LoginRequest>) -> impl Responder {
    match validate_credentials(&pool, &form.username, &form.password).await {
        Ok(valid) if valid => match generate_token(&form.username) {
            Ok(token) => HttpResponse::Ok().body(token),
            Err(_) => HttpResponse::InternalServerError().finish(),
        },
        Ok(_) => HttpResponse::Unauthorized().body("Invalid credentials"),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
fn configure_app(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/login").route(web::post().to(login)));
}
// Update the HttpServer App::new closure to use configure_app
// This struct will be used to receive the login request body
#[derive(Deserialize)]
struct LoginRequest {
    username: String,
    password: String,
}
// Update login function with database and token logic
// Assume configure_app and other necessary adjustments have been made
