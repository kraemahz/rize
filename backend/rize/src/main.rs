async fn main() {
    let routes = warp::post()
        .and(warp::path("login"))
        .and(warp::body::json())
        .and_then(handlers::login)
        .with(utilities::rate_limit())
        .with(with_logging());

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

extern crate argon2;
extern crate jsonwebtoken;
extern crate validator;
extern crate log;
extern crate env_logger;
extern crate warp;
extern crate tokio_postgres;
#[tokio::main]
mod handlers {
    use warp::http::StatusCode;
    use warp::{Rejection, Reply};

            warp::{http::StatusCode, Rejection, Reply}

    pub async fn login(body: Value) -> Result<impl Reply, Rejection> {
        let client = db::connect_to_db().await?;
        // TODO: Implement authentication logic here
        Ok(warp::reply::with_status("Login endpoint hit", StatusCode::OK))
    }
}
mod db {
    use tokio_postgres::{Client, NoTls, Error};
    use warp::Rejection;

    pub async fn connect_to_db() -> Result<Client, Rejection> {
        let (client, connection) =
            match tokio_postgres::connect("host=localhost user=postgres password=secret dbname=rizedb", NoTls).await {
                Ok((client, connection)) => {
                    tokio::spawn(async move {
                        if let Err(e) = connection.await {
                            eprintln!("connection error: {}", e);
                        }
                    });
                    Ok(client)
                },
                Err(_) => Err(warp::reject::custom(db::DbError)),
            };
        client
    }

    #[derive(Debug)]
    pub struct DbError;

    impl warp::reject::Reject for DbError {}
}
mod auth {
    use jsonwebtoken::{encode, Header, EncodingKey, errors::Result};
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize)]
    struct Claims {
        sub: String,  // Subject (whom token refers to)
        company: String,
        exp: usize,   // Expiration time (as UTC timestamp)
    }

    pub fn generate_token(user_id: &str) -> Result<String> {
        let my_claims = Claims {
            sub: user_id.to_owned(),
            company: "RizeCorp".to_owned(),
            exp: 10000000000, //TODO: replace with an appropriate expiration time
        };
        let token = encode(&Header::default(), &my_claims, &EncodingKey::from_secret("secret_key".as_ref()))?;
        Ok(token)
    }
}
mod security {
    use argon2::{self, Config};
    use rand::{Rng, distributions::Alphanumeric};

    pub fn hash_password(password: &str) -> String {
        let salt: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(16)
            .map(char::from)
            .collect();
        let config = Config::default();
        argon2::hash_encoded(password.as_bytes(), salt.as_bytes(), &config).unwrap()
    }

    pub fn verify_password(hash: &str, password: &str) -> bool {
        argon2::verify_encoded(hash, password.as_bytes()).unwrap_or(false)
    }
}
mod handlers {
    use warp::http::StatusCode;
    use warp::{Rejection, Reply};

    pub async fn login(body: serde_json::Value) -> Result<impl Reply, Rejection> {
        // TODO: Implement password hashing and verification

        Ok(warp::reply::with_status("Login endpoint hit", StatusCode::OK))
    }
}
mod utilities {
    use governor::{Quota, RateLimiter};
    use std::num::NonZeroU32;
    use std::time::Duration;
    use warp::Filter;

    pub fn rate_limit() -> impl Filter<Extract = (), Error = warp::Rejection> + Clone {
        let limiter = RateLimiter::direct(Quota::per_minute(NonZeroU32::new(30).unwrap()));

        warp::any()
            .and_then(move || {
                let limiter = limiter.clone();
                async move {
                    if let Err(_) = limiter.check_key(&()).await {
                        Err(warp::reject::custom(utilities::RateLimitError))
                    } else {
                        Ok(())
                    }
                }
            })
    }

    #[derive(Debug)]
    pub struct RateLimitError;

    impl warp::reject::Reject for RateLimitError {}
}
fn with_logging() -> impl Filter<Extract = (warp::log::Info,), Error = std::convert::Infallible> + Clone {
    warp::log("rize_api")
}