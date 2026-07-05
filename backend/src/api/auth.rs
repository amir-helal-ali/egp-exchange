use std::sync::Arc;

use axum::{Json, Router, extract::State, routing::post};
use argon2::{
    password_hash::{SaltString, rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier},
    Argon2,
};
use jsonwebtoken::{EncodingKey, Header};
use serde_json::json;

use crate::{AppState, db, errors::ApiError, models::*};

pub fn routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/auth/register", post(register))
        .route("/auth/login", post(login))
}

async fn register(
    State(state): State<Arc<AppState>>,
    Json(req): Json<RegisterRequest>,
) -> Result<Json<serde_json::Value>, ApiError> {
    if db::users::find_by_email(&state.pool, &req.email).await?.is_some() {
        return Err(ApiError::Conflict("Email already registered".into()));
    }

    let salt = SaltString::generate(&mut OsRng);
    let password_hash = Argon2::default()
        .hash_password(req.password.as_bytes(), &salt)
        .map_err(|e| ApiError::Internal(e.to_string()))?
        .to_string();

    let user = db::users::create(&state.pool, &req.email, &password_hash).await?;

    let token = generate_token(&user, &state.config.jwt_secret)?;

    Ok(Json(json!({
        "token": token,
        "user": UserPublic::from(user)
    })))
}

async fn login(
    State(state): State<Arc<AppState>>,
    Json(req): Json<LoginRequest>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let user = db::users::find_by_email(&state.pool, &req.email)
        .await?
        .ok_or_else(|| ApiError::Unauthorized)?;

    let parsed_hash = PasswordHash::new(&user.password_hash)
        .map_err(|e| ApiError::Internal(e.to_string()))?;

    Argon2::default()
        .verify_password(req.password.as_bytes(), &parsed_hash)
        .map_err(|_| ApiError::Unauthorized)?;

    let token = generate_token(&user, &state.config.jwt_secret)?;

    Ok(Json(json!({
        "token": token,
        "user": UserPublic::from(user)
    })))
}

fn generate_token(user: &User, secret: &str) -> Result<String, ApiError> {
    let now = chrono::Utc::now();
    let claims = Claims {
        sub: user.id.to_string(),
        role: user.role.clone(),
        exp: (now.timestamp() + 86400 * 7) as usize,
        iat: now.timestamp() as usize,
    };

    jsonwebtoken::encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|e| ApiError::Internal(e.to_string()))
}
