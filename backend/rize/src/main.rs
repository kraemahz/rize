use std::collections::HashMap;
use argon2::{self, Config};
use serde::{Deserialize, Serialize};
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .route("/health", web::get().to(health_check))
            .route("/login", web::post().to(login))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("Server is running")
}
#[actix_web::main]
#[derive(Deserialize)]
struct LoginRequest {
    username: String,
    password: String,
}
#[derive(Serialize)]
struct LoginResponse {
    message: String,
}
async fn login(request: web::Json<LoginRequest>) -> impl Responder {
    if verify_credentials(&request.username, &request.password) {
        return HttpResponse::Ok().json(LoginResponse {
            message: format!("Welcome, {}! You are now logged in.", request.username),
        });
    }

    HttpResponse::Unauthorized().body("Invalid username or password.")
}
// Update the HttpServer setup to include the login route
struct User {
    username: String,
    password_hash: String,
}
// Temporary in-memory 'database' for prototype purposes
lazy_static! {
    static ref USERS: HashMap<String, User> = {
        let mut m = HashMap::new();
        // An example user with a hashed password (hashing to be implemented)
        // Normally, you would not hard code users like this
        m.insert("example_user".to_string(), User {
            username: "example_user".to_string(),
            password_hash: "example_hashed_password".to_string(),
        });
        m
    };
}
// Function to simulate validating credentials against stored users
fn verify_credentials(username: &str, password: &str) -> bool {
    if let Some(user) = USERS.get(username) {
        // Compare the provided password with the stored hashed password
        return argon2::verify_encoded(&user.password_hash, password.as_bytes()).unwrap_or(false);
    }
    false
}
fn hash_password(password: &str) -> String {
    let salt = b"somesalt"; // Normally you would use a random salt.
    let config = Config::default();
    argon2::hash_encoded(password.as_bytes(), salt, &config).unwrap()
}
// A function to add a new user (hash the password and store the user)
// This is a placeholder for a real user registration process
fn add_new_user(username: String, password: String) {
    let password_hash = hash_password(&password);
    USERS.insert(username.clone(), User {
        username,
        password_hash,
    });
}