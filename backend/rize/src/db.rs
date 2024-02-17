use bcrypt::{verify, DEFAULT_COST};
use sqlx::{postgres::PgPool, Pool};

pub async fn establish_connection() -> Result<Pool, sqlx::Error> {
    dotenv::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgPool::connect(&database_url).await
}


pub async fn validate_credentials(pool: &PgPool, username: &str, password: &str) -> Result<bool, sqlx::Error> {
    let stored_user = sqlx::query_as!(User,
        "SELECT username, password_hash FROM users WHERE username = $1",
        username
    )
    .fetch_optional(pool)
    .await?;

    if let Some(user) = stored_user {
        verify(password, &user.password_hash).map_err(|_| sqlx::Error::RowNotFound)
    } else {
        Ok(false)
    }
}