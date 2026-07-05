use std::sync::Arc;

use axum::{Json, Router, extract::{State, Extension}, routing::get, middleware};
use uuid::Uuid;

use crate::{
    AppState, db, errors::ApiError, middleware::auth::require_auth,
    models::UserPublic,
};

pub fn routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/users/me", get(get_me))
        .route_layer(middleware::from_fn(require_auth))
}

async fn get_me(
    State(state): State<Arc<AppState>>,
    Extension(user_id): Extension<Uuid>,
) -> Result<Json<UserPublic>, ApiError> {
    let user = db::users::find_by_id(&state.pool, user_id)
        .await?
        .ok_or_else(|| ApiError::NotFound("User not found".into()))?;

    Ok(Json(UserPublic::from(user)))
}
