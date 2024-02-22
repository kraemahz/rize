use actix_web::{web, App, HttpResponse, HttpServer, Responder};

mod auth;

pub async fn login(credentials: web::Json<auth::Credentials>) -> impl Responder {
    match auth::authenticate_user(credentials.into_inner()) {
        Ok(_) => HttpResponse::Ok().json("Logged in successfully"),
        Err(e) => HttpResponse::Unauthorized().json(e),
    }
}

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/login").route(web::post().to(login)));
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().configure(configure_routes))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
