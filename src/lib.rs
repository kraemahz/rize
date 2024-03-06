use actix_web::{web, HttpResponse, Responder};
use bigdecimal::BigDecimal;
use bcrypt::verify;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::{PgPool, Executor};
use uuid::Uuid;


#[derive(Deserialize, Serialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub message: String,
    pub token: String,
}

pub async fn login_handler(
    _db_pool: web::Data<PgPool>, // to be used when integrating with database
    login_request: web::Json<LoginRequest>,
) -> impl Responder {
    match authenticate(&login_request.username, &login_request.password).await {
        Ok(token) => HttpResponse::Ok().json(LoginResponse {
            message: "Login successful".to_string(),
            token,
        }),
        Err(_) => HttpResponse::Unauthorized().body("Invalid credentials".to_string()),
    }
}

pub async fn authenticate(
    _username: &str, // to be used when integrating with database
    password: &str,
) -> Result<String, String> {
    // Here you would fetch the user data from the database and verify the password
    // This is a placeholder implementation
    let password_hash = "$2b$12$exSx.uDIsxJ5quCH9QB8UuBhIqyIqiSOF/cUraRNSq5ERTlG60eJm"; // Example hash
    match verify(password, password_hash) {
        Ok(valid) => {
            if valid {
                Ok("placeholder_token".to_string()) // Placeholder token generation
            } else {
                Err("Invalid credentials".to_string())
            }
        }
        Err(_) => Err("Invalid credentials".to_string()),
    }
}

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

#[derive(Serialize, Deserialize, sqlx::FromRow)]
struct Property {
    id: Uuid,
    owner_id: Uuid,
    address: String,
    city: String,
    state: String,
    zip_code: String,
    country: String,
    latitude: f64,
    longitude: f64,
    price: f64,
    availability: String,
}

pub async fn search_properties(
    pool: web::Data<PgPool>,
    web::Query(search_params): web::Query<SearchParams>,
) -> impl Responder {
    let mut query = "SELECT * FROM properties WHERE".to_string();
    let mut conditions = vec![];

    if let Some(ref address) = search_params.address {
        conditions.push(format!("address ILIKE '%{}%'", address));
    }
    // ... [additional conditions for each search parameter] ...

    if !conditions.is_empty() {
        query.push_str(&conditions.join(" AND "));
    } else {
        // Remove the " WHERE" part when there are no conditions
        query.truncate(query.len() - 6);
    }

    // Pagination
    if let Some(limit) = search_params.limit {
        query.push_str(&format!(" LIMIT {}", limit));
    }
    if let Some(offset) = search_params.offset {
        query.push_str(&format!(" OFFSET {}", offset));
    }

    let conn = pool.get_ref();

    let properties = match sqlx::query_as::<_, Property>(&query)
        .fetch_all(conn)
        .await {
        Ok(props) => props,
        Err(_) => {
            return HttpResponse::InternalServerError().json(
                json!({"error": "Failed to query for properties"})
            );
        }
    };
    HttpResponse::Ok().json(properties)
}
