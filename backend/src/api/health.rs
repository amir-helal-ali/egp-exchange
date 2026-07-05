use std::sync::Arc;

use axum::{Json, Router, extract::State, routing::get};

use crate::AppState;

pub fn routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/health", get(health_check))
}

async fn health_check(
    State(state): State<Arc<AppState>>,
) -> Json<serde_json::Value> {
    let circuit_open = state.price_feed.read().await.circuit_open;
    Json(serde_json::json!({
        "status": if circuit_open { "degraded" } else { "ok" },
        "circuit_breaker": circuit_open,
    }))
}
