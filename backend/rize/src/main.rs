async fn main() {
    env_logger::init();
    let routes = warp::post()
        .and(warp::path("login"))
        .and(warp::body::json())
        .and_then(handlers::login);

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

    pub async fn login(body: serde_json::Value) -> Result<impl Reply, Rejection> {
        // TODO: Implement login logic here
        Ok(warp::reply::with_status("Login endpoint hit", StatusCode::OK))
    }
}