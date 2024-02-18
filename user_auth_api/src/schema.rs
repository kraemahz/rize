use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow)]
struct User {
    id: i32,
    username: String,
    password_hash: String,
}