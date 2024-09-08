use axum::{Router, routing::post, Json, extract::Extension};
use super::password::verify_password;
use super::jwt::create_jwt;
use serde::Deserialize;
use sqlx::PgPool;

#[derive(Deserialize)]
struct LoginRequest {
    email: String,
    password: String,
}

pub async fn login(
    Json(payload): Json<LoginRequest>, 
    Extension(pool): Extension<PgPool>
) -> Result<Json<String>, &'static str> {
    let user = sqlx::query!(
        "SELECT id, password_hash FROM users WHERE email = $1",
        payload.email
    )
    .fetch_one(&pool)
    .await
    .map_err(|_| "Invalid email or password")?;

    if verify_password(&user.password_hash, &payload.password) {
        let token = create_jwt(user.id.to_string());
        Ok(Json(token))
    } else {
        Err("Invalid email or password")
    }
}

pub fn auth_routes() -> Router {
    Router::new().route("/login", post(login))
}
