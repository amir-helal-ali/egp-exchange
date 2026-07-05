use axum::{
    Extension,
    body::Body,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};
use jsonwebtoken::{DecodingKey, Validation, decode};
use uuid::Uuid;

use crate::models::Claims;

pub async fn require_auth(
    mut req: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    let token = req
        .headers()
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "super-secret-key-change-me".into());

    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .map_err(|_| StatusCode::UNAUTHORIZED)?;

    let user_id = Uuid::parse_str(&token_data.claims.sub)
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    req.extensions_mut().insert(user_id);
    req.extensions_mut().insert(token_data.claims.role.clone());

    Ok(next.run(req).await)
}

pub async fn require_admin(
    Extension(role): Extension<String>,
    req: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    if role != "admin" {
        return Err(StatusCode::FORBIDDEN);
    }
    Ok(next.run(req).await)
}
