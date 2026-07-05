pub mod auth;
pub mod users;
pub mod wallets;
pub mod orders;
pub mod admin;
pub mod health;
pub mod futures;
pub mod p2p;

use axum::{Router, extract::{State, ws::WebSocketUpgrade}, response::IntoResponse, routing::get};
use std::sync::Arc;
use tower_http::cors::CorsLayer;

use crate::AppState;
use crate::ws::handle_socket;

pub fn router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/ws", get(ws_handler))
        .nest("/api/v1", api_routes())
        .with_state(state)
        .layer(CorsLayer::permissive())
}

async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, state.ws_pubsub.clone()))
}

fn api_routes() -> Router<Arc<AppState>> {
    Router::new()
        .merge(auth::routes())
        .merge(users::routes())
        .merge(wallets::routes())
        .merge(orders::routes())
        .merge(admin::routes())
        .merge(health::routes())
        .merge(futures::routes())
        .merge(p2p::routes())
}
