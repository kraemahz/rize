use uuid::Uuid;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().configure(configure)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

async fn login(credentials: web::Json<Credentials>) -> impl Responder {
    // Mock user data
    let user = User {
        id: Uuid::new_v4(),
        username: "example_user".to_string(),
        hashed_password: hash("example_password", bcrypt::DEFAULT_COST).unwrap(),
    };

    if user.username == credentials.username && user.verify_password(&credentials.password) {
        let token = generate_jwt(user.username.clone(), 24);
        HttpResponse::Ok().json(token)
    } else {
        HttpResponse::Unauthorized().finish()
    }
}
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/login").route(web::post().to(login)),
    );
}
#[derive(serde::Deserialize)]
struct Credentials {
    username: String,
    password: String,
}