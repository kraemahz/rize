use actix_web::{web, HttpResponse};
fn main() {
    println!("Hello, world!");
}

async fn login(data: web::Json<LoginRequest>) -> impl Responder {
    // Placeholder for the actual login logic
    "Login endpoint reached"
}
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/login").route(web::post().to(login)));
}
#[derive(serde::Deserialize)]
struct LoginRequest {
    username: String,
    password: String,
}
