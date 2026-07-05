pub mod auth;
pub mod users;
pub mod wallets;
pub mod orders;
pub mod admin;
pub mod health;

use axum::Router;
use std::sync::Arc;
use tower_http::cors::CorsLayer;

use crate::AppState;

pub fn router(state: Arc<AppState>) -> Router {
    Router::new()
        .nest("/api/v1", api_routes())
        .with_state(state)
        .layer(CorsLayer::permissive())
}

fn api_routes() -> Router<Arc<AppState>> {
    Router::new()
        .merge(auth::routes())
        .merge(users::routes())
        .merge(wallets::routes())
        .merge(orders::routes())
        .merge(admin::routes())
        .merge(health::routes())
}
