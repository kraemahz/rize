use actix_web::{web, HttpResponse};
use argon2::{self, Config};
use bcrypt::{hash, verify};
use postgres::{Client, NoTls};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
struct Credentials {
    username: String,
    password: String,
}

pub fn authenticate_user(credentials: Credentials) -> Result<(), &'static str> {
    let mut client = Client::connect("host=localhost user=postgres", NoTls).unwrap();

    let stmt = client
        .prepare("SELECT password FROM users WHERE username = $1")
        .unwrap();
    let rows = client.query(&stmt, &[&credentials.username]).unwrap();

    if let Some(row) = rows.iter().next() {
        let hashed_password: String = row.get(0);

        if verify_password(&credentials.password, &hashed_password) {
            Ok(())
        } else {
            Err("Invalid username or password")
        }
    } else {
        Err("Invalid username or password")
    }
}
pub fn hash_password(password: &str) -> String {
    let salt = b"somesalt"; // NOTE: use a proper random salt in production
    let config = Config::default();
    argon2::hash_encoded(password.as_bytes(), salt, &config).unwrap()
}
pub fn verify_password(password: &str, hash: &str) -> bool {
    argon2::verify_encoded(hash, password.as_bytes()).unwrap_or(false)
}
