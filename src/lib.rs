use actix_web::{HttpServer, App, web, Responder, HttpResponse};
use bigdecimal::BigDecimal;
use serde::Deserialize;
use sqlx::PgPool;

#[derive(Deserialize)]
pub struct SearchParams {
    address: Option<String>,
    city: Option<String>,
    state: Option<String>,
    zip_code: Option<String>,
    country: Option<String>,
    latitude: Option<f64>,
    longitude: Option<f64>,
    price_min: Option<BigDecimal>,
    price_max: Option<BigDecimal>,
    availability: Option<String>,
    limit: Option<i64>,
    offset: Option<i64>,
}

pub async fn search_properties(
    _pool: web::Data<PgPool>,
    web::Query(_search_params): web::Query<SearchParams>,
) -> impl Responder {
    // TODO: Implement search logic
    HttpResponse::Ok().json("Search endpoint reached")
}

async fn _search_properties(
    _pool: web::Data<PgPool>,
    web::Query(_search_params): web::Query<SearchParams>,
) -> impl Responder {
    // TODO: Implement search logic
    HttpResponse::Ok().json("Search endpoint reached")
}
