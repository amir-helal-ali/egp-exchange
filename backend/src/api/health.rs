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
    let feed = state.price_feed.read().await;
    tracing::debug!("Health check: last={:?}, bid={:?}, ask={:?}", feed.last, feed.bid, feed.ask);
    Json(serde_json::json!({
        "status": if feed.circuit_open { "degraded" } else { "ok" },
        "circuit_breaker": feed.circuit_open,
        "last_price": feed.last,
    }))
}
