use uuid::Uuid;
use bcrypt::{verify};

    bcrypt::verify
use sqlx::postgres::PgPoolOptions;
use serde::{Deserialize, Serialize};
use actix_web::{web, App, HttpServer, Responder, HttpResponse, post};

    actix_web::{web, App, HttpServer, Responder, HttpResponse, post, http::StatusCode}

    serde::{Deserialize, Serialize}

    actix_web::{web, App, HttpServer, HttpResponse, post}

    actix_web::{web, App, HttpServer, Responder, HttpResponse, post, http::StatusCode}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await.expect("Failed to connect to the database");

    let pool = web::Data::new(pool);

    HttpServer::new(move || {
        App::new()
            .app_data(pool.clone())
            .service(login)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

#[post("/login")]
async fn login(pool: web::Data<PgPool>, info: web::Json<LoginRequest>) -> impl Responder {
    let login_info = info.into_inner();
    let user = match sqlx::query_as!(User, "SELECT * FROM users WHERE username = $1", &login_info.username)
        .fetch_one(&*pool).await {
            Ok(user) => user,
            Err(_) => return HttpResponse::Unauthorized().json("Invalid username or password"),
        };

    if verify(&login_info.password, &user.password_hash).unwrap_or(false) {
        let token = generate_session_token();
        HttpResponse::Ok().json(LoginResponse { token })
    } else {
        HttpResponse::Unauthorized().json("Invalid password")
    }
}    HttpResponse::Unauthorized().body("Invalid username or password")
    }
}
#[derive(Deserialize, Debug)]
struct LoginRequest {
    username: String,
    password: String,
}
;
#[derive(Serialize)]
struct LoginResponse {
    token: String,
}
async fn authenticate(username: &str, password: &str) -> bool {
    // TODO: Implement authentication logic with the database
    false
}
async fn generate_session_token() -> String {
    Uuid::new_v4().to_string()
}
// Define User struct to hold data fetched from the database
#[derive(sqlx::FromRow)]
struct User {
    id: i32,
    username: String,
    password_hash: String,
}
;