#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, web, App, HttpResponse, Responder};
    use sqlx::{Pool, Postgres};
    use std::env;
    use uuid::Uuid;

    // Function to create a test reservation request
    fn create_test_reservation_request(user_id: Uuid) -> ReservationRequest {
        ReservationRequest {
            user_id,
            start_date: NaiveDate::from_ymd_opt(2100, 1, 1).expect("date"),
            end_date: NaiveDate::from_ymd_opt(2100, 1, 10).expect("date"),
        }
    }

    // Test for successful reservation creation
    #[actix_rt::test]
    async fn test_reserve_property_success() {
        let pgpassword = env::var("PGPASSWORD").expect("must have secret");
        let pgstring = format!("postgres://postgres:{}@postgres.postgres/test", pgpassword);

        let pool = Pool::<Postgres>::connect(&pgstring)
            .await
            .unwrap();

        let user_id = Uuid::new_v4();
        let property_id = Uuid::new_v4();

        sqlx::query!("INSERT INTO users (id, username) VALUES ($1, 'test')", user_id)
            .execute(&pool)
            .await.expect("inserted user");
        sqlx::query!("INSERT INTO properties (id, owner_id) VALUES ($1, $2)", property_id, user_id)
            .execute(&pool)
            .await.expect("inserted property");

        let service = test::init_service(App::new().app_data(web::Data::new(pool)).route(
            "/properties/{propertyId}/reservations",
            web::post().to(reserve_property),
        ))
        .await;
        let req = test::TestRequest::post()
            .uri(&format!("/properties/{}/reservations", property_id))
            .set_json(&create_test_reservation_request(user_id))
            .to_request();
        let resp = test::call_service(&service, req).await;

        assert!(resp.status().is_success());
    }

    // TODO: Add more tests for error cases (e.g., invalid input, reservation conflicts)
}
use actix_web::{web, HttpResponse, HttpServer, App, Responder};
use chrono::{NaiveDate, Datelike};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::env;
use sqlx::{PgPool, Pool, Postgres};
use rize::search_properties;

async fn reserve_property(
    pool: web::Data<PgPool>,
    path: web::Path<(Uuid,)>,
    json: web::Json<ReservationRequest>,
) -> impl Responder {
    let property_id = path.into_inner().0;
    let request = json.into_inner();

    // Validate the reservation request
    if let Err(response) = validate_reservation_request(&request) {
        return response;
    }

    // Check for existing reservations and handle conflicts
    if let Err(response) = check_reservation_conflicts(&pool, property_id, &request).await {
        return response;
    }

    // TODO: Persist the reservation to the database

    HttpResponse::Ok().json("Reservation successful") // Placeholder
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pgpassword = env::var("PGPASSWORD").expect("must have secret");
    let pgstring = format!("postgres://postgres:{}@postgres.postgres/test", pgpassword);

    let pool = Pool::<Postgres>::connect(&pgstring)
        .await
        .unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route(
                "/properties/{propertyId}/reservations",
                web::post().to(reserve_property),
            )
            .route(
                "/api/properties/search",
                web::get().to(search_properties),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[derive(Serialize, Deserialize)]
struct ReservationRequest {
    user_id: Uuid,
    start_date: NaiveDate,
    end_date: NaiveDate,
}

fn validate_reservation_request(request: &ReservationRequest) -> Result<(), HttpResponse> {
    if request.start_date >= request.end_date {
        Err(HttpResponse::BadRequest().body("End date must be after start date"))
    } else if request.start_date
        < NaiveDate::from_ymd(
            chrono::Local::now().year(),
            chrono::Local::now().month(),
            chrono::Local::now().day(),
        )
    {
        Err(HttpResponse::BadRequest().body("Start date must be in the future"))
    } else {
        Ok(()) // Validation passes
    }
}

async fn check_reservation_conflicts(
    pool: &PgPool,
    property_id: Uuid,
    request: &ReservationRequest,
) -> Result<(), HttpResponse> {
    // Query to find any reservation that conflicts with the requested dates
    let conflict_query = "SELECT * FROM reservations WHERE property_id = $1 AND NOT (end_date <= $2 OR start_date >= $3)";
    let conflicts = sqlx::query_as::<_, Reservation>(conflict_query)
        .bind(property_id)
        .bind(request.start_date)
        .bind(request.end_date)
        .fetch_optional(pool)
        .await
        .map_err(|_| {
            HttpResponse::InternalServerError().body("Failed to query for existing reservations")
        })?;

    // If there is a conflict, return an error response
    if conflicts.is_some() {
        return Err(HttpResponse::Conflict()
            .body("Reservation dates conflict with an existing reservation"));
    }

    // No conflicts, reservation is possible
    Ok(())
}

#[derive(sqlx::FromRow)]
struct Reservation {
    id: Uuid,
    property_id: Uuid,
    user_id: Uuid,
    start_date: NaiveDate,
    end_date: NaiveDate,
}

#[derive(sqlx::FromRow)]
struct User {
    id: Uuid,
    username: String
}

#[derive(sqlx::FromRow)]
struct Property {
    id: Uuid,
    owner_id: Uuid
}

async fn persist_reservation(
    pool: &PgPool,
    property_id: &str,
    request: &ReservationRequest,
) -> Result<i32, HttpResponse> {
    // Assuming reservation id is of type i32
    let insert_query = "INSERT INTO reservations (property_id, user_id, start_date, end_date) VALUES ($1, $2, $3, $4) RETURNING id";
    let reservation_id: i32 = sqlx::query_scalar(insert_query)
        .bind(property_id)
        .bind(&request.user_id)
        .bind(request.start_date)
        .bind(request.end_date)
        .fetch_one(pool)
        .await
        .map_err(|_| {
            HttpResponse::InternalServerError().body("Failed to insert new reservation")
        })?;

    Ok(reservation_id) // Return the id of the new reservation
}
