mod api;
mod binance;
mod config;
mod db;
mod engine;
mod errors;
mod middleware;
mod models;
mod redis;

use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::Mutex;
use tower_http::trace::TraceLayer;
use tracing_subscriber::EnvFilter;

use crate::binance::ws::{BinanceConfig, PriceFeedState, start_price_feed};
use crate::config::Config;
use crate::engine::matching::MatchingEngine;

pub struct AppState {
    pub pool: sqlx::PgPool,
    pub redis: ::redis::aio::ConnectionManager,
    pub config: Config,
    pub price_feed: PriceFeedState,
    pub engines: Mutex<HashMap<(String, String), MatchingEngine>>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()))
        .init();

    let config = Config::from_env();

    let pool = db::connect(&config.database_url).await?;
    db::migrations::run_migrations(&pool).await?;

    let redis_mgr = redis::connect(&config.redis_url).await?;

    let binance_cfg = BinanceConfig {
        ws_base: config.binance_ws_url.clone(),
        ..Default::default()
    };
    let (_price_feed, _price_rx) = start_price_feed(binance_cfg).await;

    let state = Arc::new(AppState {
        pool,
        redis: redis_mgr,
        config: config.clone(),
        price_feed: _price_feed,
        engines: Mutex::new(HashMap::new()),
    });

    // Bootstrap admin user
    let admin_email = std::env::var("ADMIN_EMAIL").unwrap_or_else(|_| "admin@egp.exchange".into());
    let admin_pass = std::env::var("ADMIN_PASSWORD").unwrap_or_else(|_| "admin123".into());

    if db::users::find_by_email(&state.pool, &admin_email)
        .await?
        .is_none()
    {
        use argon2::password_hash::{SaltString, rand_core::OsRng, PasswordHasher};
        use argon2::Argon2;
        let salt = SaltString::generate(&mut OsRng);
        let hash = Argon2::default()
            .hash_password(admin_pass.as_bytes(), &salt)
            .unwrap()
            .to_string();

        sqlx::query(
            "INSERT INTO users (email, password_hash, role) VALUES ($1, $2, 'admin')",
        )
        .bind(&admin_email)
        .bind(&hash)
        .execute(&state.pool)
        .await?;

        tracing::info!("Admin user created: {admin_email}");
    }

    let app = api::router(state.clone()).layer(TraceLayer::new_for_http());

    let addr = config.server_addr();
    tracing::info!("Server starting on {addr}");

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
