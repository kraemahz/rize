use crate::UserProfile;
use actix_web::{web, HttpResponse, Responder};
use serde_json::json;
use sqlx::{query_as, PgPool};
#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct UserProfile {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub avatar_url: Option<String>,
}

pub async fn create_user_profile(
    pool: web::Data<PgPool>,
    new_profile: web::Json<UserProfile>,
) -> impl Responder {
    let result = sqlx::query!(
        "INSERT INTO user_profiles (username, email, first_name, last_name, avatar_url) VALUES ($1, $2, $3, $4, $5) RETURNING id",
        new_profile.username,
        new_profile.email,
        new_profile.first_name,
        new_profile.last_name,
        new_profile.avatar_url,
    )
    .fetch_one(&**pool)
    .await;

    match result {
        Ok(record) => HttpResponse::Created().json(record),
        Err(e) => HttpResponse::BadRequest().body(e.to_string()),
    }
}
pub async fn read_user_profile(pool: web::Data<PgPool>, user_id: web::Path<i32>) -> impl Responder {
    let result = sqlx::query_as!(
        UserProfileData,
        "SELECT * FROM user_profiles WHERE id = $1",
        user_id
    )
    .fetch_one(&**pool)
    .await;

    match result {
        Ok(profile) => HttpResponse::Ok().json(profile),
        Err(e) => HttpResponse::NotFound().body(e.to_string()),
    }
}
pub async fn update_user_profile(
    pool: web::Data<PgPool>,
    user_id: web::Path<i32>,
    profile_update: web::Json<UserProfile>,
) -> impl Responder {
    let result = sqlx::query!(
        "UPDATE user_profiles SET username = $1, email = $2, first_name = $3, last_name = $4, avatar_url = $5 WHERE id = $6",
        profile_update.username,
        profile_update.email,
        profile_update.first_name,
        profile_update.last_name,
        profile_update.avatar_url,
        user_id.into_inner(),
    )
    .execute(&**pool)
    .await;

    match result {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => HttpResponse::BadRequest().body(e.to_string()),
    }
}
pub async fn delete_user_profile(
    pool: web::Data<PgPool>,
    user_id: web::Path<i32>,
) -> impl Responder {
    let result = sqlx::query!(
        "DELETE FROM user_profiles WHERE id = $1",
        user_id.into_inner()
    )
    .execute(&**pool)
    .await;

    match result {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => HttpResponse::BadRequest().body(e.to_string()),
    }
}
pub async fn insert_user_profile(pool: &PgPool, profile: &UserProfile) -> sqlx::Result<i32> {
    let mut tx = pool.begin().await?;
    let user_id = query!(
        "INSERT INTO user_profiles (username, email, first_name, last_name, avatar_url) VALUES ($1, $2, $3, $4, $5) RETURNING id",
        &profile.username, &profile.email, &profile.first_name, &profile.last_name, &profile.avatar_url
    )
    .fetch_one(&mut tx)
    .await?
    .id;
    tx.commit().await?;
    Ok(user_id)
}
pub async fn select_user_profile_by_id(pool: &PgPool, user_id: i32) -> sqlx::Result<UserProfile> {
    let profile = query_as!(
        UserProfile,
        "SELECT * FROM user_profiles WHERE id = $1",
        user_id
    )
    .fetch_one(pool)
    .await?;
    Ok(profile)
}
pub async fn update_user_profile_by_id(
    pool: &PgPool,
    user_id: i32,
    profile: &UserProfile,
) -> sqlx::Result<u64> {
    let rows_affected = query!(
        "UPDATE user_profiles SET username = $1, email = $2, first_name = $3, last_name = $4, avatar_url = $5 WHERE id = $6",
        &profile.username, &profile.email, &profile.first_name, &profile.last_name, &profile.avatar_url, user_id
    )
    .execute(pool)
    .await?
    .rows_affected();
    Ok(rows_affected)
}
pub async fn delete_user_profile_by_id(pool: &PgPool, user_id: i32) -> sqlx::Result<u64> {
    let rows_affected = query!("DELETE FROM user_profiles WHERE id = $1", user_id)
        .execute(pool)
        .await?
        .rows_affected();
    Ok(rows_affected)
}
#[derive(sqlx::FromRow)]
struct UserProfileData {
    id: i32,
    username: String,
    email: String,
    first_name: Option<String>,
    last_name: Option<String>,
    avatar_url: Option<String>,
}
