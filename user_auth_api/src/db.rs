use sqlx::{postgres::PgPoolOptions, PgPool};

pub async fn establish_connection(database_url: &str) -> Result<PgPool, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await
}
