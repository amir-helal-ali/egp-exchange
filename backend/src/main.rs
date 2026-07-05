mod admin_controls;
mod api;
mod binance;
mod config;
mod db;
mod engine;
mod errors;
mod futures;
mod middleware;
mod models;
mod p2p;
mod redis;
mod ws;

use std::collections::HashMap;
use std::sync::Arc;

use rust_decimal::Decimal;
use tokio::sync::Mutex;
use tower_http::trace::TraceLayer;
use tracing_subscriber::EnvFilter;

use crate::engine::orderbook::Order as EngineOrder;

use crate::binance::ws::{BinanceConfig, PriceFeedState, start_price_feed};
use crate::config::Config;
use crate::engine::matching::MatchingEngine;

pub struct AppState {
    pub pool: sqlx::PgPool,
    pub redis: ::redis::aio::ConnectionManager,
    pub config: Config,
    pub price_feed: PriceFeedState,
    pub engines: Mutex<HashMap<(String, String), MatchingEngine>>,
    pub ws_pubsub: crate::ws::PubSub,
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

    let ws_pubsub = crate::ws::PubSub::new();

    let state = Arc::new(AppState {
        pool,
        redis: redis_mgr,
        config: config.clone(),
        price_feed: _price_feed,
        engines: Mutex::new(HashMap::new()),
        ws_pubsub,
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

    // Background futures liquidation + mark price update + TP/SL check
    let liq_state = state.clone();
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(std::time::Duration::from_secs(5));
        loop {
            interval.tick().await;
            if let Ok(positions) = db::futures::find_all_open(&liq_state.pool).await {
                let price = liq_state.price_feed.read().await.last;
                if let Some(mark_price) = price {
                    for pos in &positions {
                        // Update mark price and unrealized PnL
                        let pnl = futures::FuturesEngine::calculate_pnl(
                            pos.entry_price, mark_price, pos.quantity, &pos.side,
                        );
                        let _ = db::futures::update_price(&liq_state.pool, pos.id, mark_price, pnl).await;

                        if let Some(c) = futures::FuturesEngine::check_liquidation(
                            &liq_state.pool, pos, mark_price,
                        ).await {
                            let _ = liq_state.ws_pubsub.publish_json(
                                &format!("positions:{}", c.user_id),
                                &serde_json::json!({
                                    "channel": format!("positions:{}", c.user_id),
                                    "data": serde_json::json!({"action": "liquidated", "id": c.id}),
                                }),
                            ).await;
                        }
                        if let Some(c) = futures::FuturesEngine::check_tp_sl(
                            &liq_state.pool, pos, mark_price,
                        ).await {
                            let _ = liq_state.ws_pubsub.publish_json(
                                &format!("positions:{}", c.user_id),
                                &serde_json::json!({
                                    "channel": format!("positions:{}", c.user_id),
                                    "data": serde_json::json!({"action": "tp_sl_closed", "id": c.id}),
                                }),
                            ).await;
                        }
                    }
                }
            }
        }
    });

    // Background stop-limit order checker
    let stop_state = state.clone();
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(std::time::Duration::from_secs(3));
        loop {
            interval.tick().await;
            let price = stop_state.price_feed.read().await.last;
            let mark_price = match price {
                Some(p) => p,
                None => continue,
            };

            let pairs: Vec<(String, String)> = {
                let engines = stop_state.engines.lock().await;
                engines.keys().cloned().collect()
            };

            for (base, quote) in &pairs {
                if let Ok(orders) = db::orders::find_stop_orders(&stop_state.pool, base, quote).await {
                    for order in &orders {
                        let triggered = match order.side.as_str() {
                            "buy" => order.stop_price.is_some_and(|sp| mark_price <= sp),
                            "sell" => order.stop_price.is_some_and(|sp| mark_price >= sp),
                            _ => false,
                        };
                        if !triggered { continue; }

                        if let Ok(triggered_order) = db::orders::update_trigger(&stop_state.pool, order.id).await {
                            let engine_order = EngineOrder {
                                id: triggered_order.id,
                                user_id: triggered_order.user_id,
                                side: triggered_order.side.clone(),
                                price: triggered_order.price,
                                quantity: triggered_order.quantity - triggered_order.filled,
                                filled: triggered_order.filled,
                            };
                            let mut engines = stop_state.engines.lock().await;
                            if let Some(engine) = engines.get_mut(&(base.clone(), quote.clone())) {
                                let (trades, _) = engine.process_limit(&engine_order);
                                drop(engines);

                                for trade in &trades {
                                    let _ = db::trades::insert(
                                        &stop_state.pool,
                                        trade.buy_order_id, trade.sell_order_id,
                                        &trade.base_currency, &trade.quote_currency,
                                        trade.price, trade.quantity, trade.total,
                                        &trade.taker_side,
                                    ).await;
                                    let _ = db::wallets::settle_lock(
                                        &stop_state.pool, trade.buy_user_id, &quote, trade.total,
                                    ).await;
                                    let _ = db::wallets::ensure_wallet(&stop_state.pool, trade.buy_user_id, base).await;
                                    let _ = db::wallets::add_balance(
                                        &stop_state.pool, trade.buy_user_id, base, trade.quantity,
                                    ).await;
                                    let _ = db::wallets::ensure_wallet(&stop_state.pool, trade.sell_user_id, quote).await;
                                    let _ = db::wallets::add_balance(
                                        &stop_state.pool, trade.sell_user_id, quote, trade.total,
                                    ).await;
                                    let _ = db::wallets::settle_lock(
                                        &stop_state.pool, trade.sell_user_id, base, trade.quantity,
                                    ).await;
                                }
                                let filled_qty: Decimal = trades.iter().map(|t| t.quantity).sum();
                                let status = if filled_qty >= triggered_order.quantity { "filled" } else { "partial" };
                                let _ = db::orders::update_filled(&stop_state.pool, triggered_order.id, filled_qty, status).await;

                                // Publish trades
                                if !trades.is_empty() {
                                    let trade_channel = format!("trades:{}{}", base, quote);
                                    let trade_data: Vec<serde_json::Value> = trades.iter().map(|t| {
                                        serde_json::json!({
                                            "price": t.price,
                                            "quantity": t.quantity,
                                            "total": t.total,
                                            "base_currency": t.base_currency,
                                            "quote_currency": t.quote_currency,
                                            "taker_side": t.taker_side,
                                            "created_at": chrono::Utc::now(),
                                        })
                                    }).collect();
                                    let _ = stop_state.ws_pubsub.publish_json(&trade_channel, &serde_json::json!({
                                        "channel": trade_channel,
                                        "data": trade_data,
                                    })).await;
                                }
                            }
                        }
                    }
                }
            }
        }
    });

    let app = api::router(state.clone()).layer(TraceLayer::new_for_http());

    let addr = config.server_addr();
    tracing::info!("Server starting on {addr}");

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
