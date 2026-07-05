use std::sync::Arc;

use axum::{
    Extension, Json, Router,
    extract::{Path, State},
    middleware,
    routing::{delete, get, post, put},
};
use serde_json::Value;
use uuid::Uuid;

use crate::{
    AppState, db, errors::ApiError,
    middleware::auth::{require_auth, require_admin},
    models::*,
    redis::queue::{self, QueueItem},
};

pub fn routes() -> Router<Arc<AppState>> {
    Router::new()
        // Transaction processing
        .route("/admin/transactions/pending", get(pending_transactions))
        .route("/admin/transactions/:id", post(process_transaction))
        // Users
        .route("/admin/users", get(list_users))
        .route("/admin/wallets", get(all_wallets))
        // Liquidity
        .route("/admin/liquidity", get(liquidity))
        // Currencies
        .route("/admin/currencies", get(list_currencies))
        .route("/admin/currencies", post(create_currency))
        .route("/admin/currencies/:code", put(update_currency))
        .route("/admin/currencies/:code/toggle", post(toggle_currency))
        .route("/admin/currencies/:code", delete(delete_currency))
        // Trading pairs
        .route("/admin/pairs", get(list_pairs))
        .route("/admin/pairs", post(create_pair))
        .route("/admin/pairs/:id", put(update_pair))
        .route("/admin/pairs/:id", delete(delete_pair))
        // Settings
        .route("/admin/settings", get(list_settings))
        .route("/admin/settings/:key", put(set_setting))
        // Futures admin
        .route("/admin/futures/positions", get(all_futures_positions))
        .route("/admin/futures/positions/:id/close", post(force_close_position))
        // P2P admin
        .route("/admin/p2p/orders", get(all_p2p_orders))
        .route("/admin/p2p/orders/:id/resolve", post(resolve_dispute))
        .route_layer(middleware::from_fn(require_admin))
        .route_layer(middleware::from_fn(require_auth))
}

// --- Transaction processing ---

async fn pending_transactions(
    State(state): State<Arc<AppState>>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let mut redis = state.redis.clone();
    let deposits: Vec<QueueItem> = queue::get_pending_deposits(&mut redis).await?;
    let withdrawals: Vec<QueueItem> = queue::get_pending_withdrawals(&mut redis).await?;

    Ok(Json(serde_json::json!({
        "deposits": deposits,
        "withdrawals": withdrawals
    })))
}

async fn process_transaction(
    State(state): State<Arc<AppState>>,
    Extension(admin_id): Extension<Uuid>,
    Path(id): Path<Uuid>,
    Json(req): Json<ProcessTransactionRequest>,
) -> Result<Json<ManualTransaction>, ApiError> {
    let tx = db::manual_transactions::find_by_id(&state.pool, id)
        .await?
        .ok_or_else(|| ApiError::NotFound("Transaction not found".into()))?;

    if tx.status != "pending" {
        return Err(ApiError::BadRequest("Transaction already processed".into()));
    }

    match req.action.as_str() {
        "approve" => {
            let processed = db::manual_transactions::process(
                &state.pool, id, admin_id, "approved", req.notes.as_deref(),
            ).await?;

            match tx.tx_type.as_str() {
                "deposit" => {
                    let _ = db::wallets::ensure_wallet(&state.pool, tx.user_id, &tx.currency).await?;
                    let _ = db::wallets::add_balance(&state.pool, tx.user_id, &tx.currency, tx.amount).await;
                }
                "withdrawal" => {
                    let _ = db::wallets::add_balance(&state.pool, tx.user_id, &tx.currency, -tx.amount).await;
                }
                _ => return Err(ApiError::BadRequest("Invalid transaction type".into()))
            }

            let mut redis = state.redis.clone();
            match tx.tx_type.as_str() {
                "deposit" => queue::remove_deposit(&mut redis, tx.id).await?,
                "withdrawal" => queue::remove_withdrawal(&mut redis, tx.id).await?,
                _ => return Err(ApiError::BadRequest("Invalid transaction type".into()))
            }

            Ok(Json(processed))
        }
        "reject" => {
            let processed = db::manual_transactions::process(
                &state.pool, id, admin_id, "rejected", req.notes.as_deref(),
            ).await?;

            let mut redis = state.redis.clone();
            match tx.tx_type.as_str() {
                "deposit" => queue::remove_deposit(&mut redis, tx.id).await?,
                "withdrawal" => queue::remove_withdrawal(&mut redis, tx.id).await?,
                _ => return Err(ApiError::BadRequest("Invalid transaction type".into()))
            }

            Ok(Json(processed))
        }
        _ => Err(ApiError::BadRequest("Action must be 'approve' or 'reject'".into())),
    }
}

// --- Users ---

async fn list_users(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<UserPublic>>, ApiError> {
    let users = db::users::list_all(&state.pool).await?;
    Ok(Json(users.into_iter().map(UserPublic::from).collect()))
}

async fn all_wallets(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<Wallet>>, ApiError> {
    let wallets = db::wallets::list_all(&state.pool).await?;
    Ok(Json(wallets))
}

// --- Liquidity ---

async fn liquidity(
    State(state): State<Arc<AppState>>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let engines = state.engines.lock().await;
    let mut pairs = Vec::new();
    for ((base, quote), engine) in engines.iter() {
        let ob = engine.orderbook.snapshot(5);
        let bid_total: rust_decimal::Decimal = ob.bids.iter().map(|l| l.price * l.quantity).sum();
        let ask_total: rust_decimal::Decimal = ob.asks.iter().map(|l| l.price * l.quantity).sum();
        pairs.push(serde_json::json!({
            "pair": format!("{}/{}", base, quote),
            "bid_depth": bid_total,
            "ask_depth": ask_total,
            "bid_count": ob.bids.len(),
            "ask_count": ob.asks.len(),
            "best_bid": ob.bids.first().map(|l| l.price),
            "best_ask": ob.asks.first().map(|l| l.price),
        }));
    }
    drop(engines);

    Ok(Json(serde_json::json!({
        "pairs": pairs,
        "circuit_breaker": state.price_feed.read().await.circuit_open,
    })))
}

// --- Currencies ---

async fn list_currencies(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<Currency>>, ApiError> {
    let currencies = db::settings::currencies::list_all(&state.pool).await?;
    Ok(Json(currencies))
}

async fn create_currency(
    State(state): State<Arc<AppState>>,
    Json(req): Json<CreateCurrencyRequest>,
) -> Result<Json<Currency>, ApiError> {
    let currency = crate::admin_controls::create_currency(&state.pool, req).await?;
    Ok(Json(currency))
}

async fn update_currency(
    State(state): State<Arc<AppState>>,
    Path(code): Path<String>,
    Json(req): Json<CreateCurrencyRequest>,
) -> Result<Json<Currency>, ApiError> {
    let mut create_req = req;
    create_req.code = code;
    let currency = crate::admin_controls::update_currency(&state.pool, create_req).await?;
    Ok(Json(currency))
}

async fn toggle_currency(
    State(state): State<Arc<AppState>>,
    Path((code, feature)): Path<(String, String)>,
    Json(req): Json<serde_json::Value>,
) -> Result<Json<Currency>, ApiError> {
    let enabled = req["enabled"].as_bool().unwrap_or(true);
    let currency = crate::admin_controls::toggle_currency_feature(
        &state.pool, &code, &feature, enabled,
    ).await?;
    Ok(Json(currency))
}

async fn delete_currency(
    State(state): State<Arc<AppState>>,
    Path(code): Path<String>,
) -> Result<Json<serde_json::Value>, ApiError> {
    db::settings::currencies::delete(&state.pool, &code).await?;
    Ok(Json(serde_json::json!({"status": "deleted", "code": code })))
}

// --- Trading Pairs ---

async fn list_pairs(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<TradingPair>>, ApiError> {
    let pairs = db::settings::trading_pairs::list_all(&state.pool).await?;
    Ok(Json(pairs))
}

async fn create_pair(
    State(state): State<Arc<AppState>>,
    Json(req): Json<CreateTradingPairRequest>,
) -> Result<Json<TradingPair>, ApiError> {
    let pair = crate::admin_controls::create_trading_pair(&state.pool, req).await?;
    Ok(Json(pair))
}

async fn update_pair(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    Json(req): Json<CreateTradingPairRequest>,
) -> Result<Json<TradingPair>, ApiError> {
    let pair = crate::admin_controls::update_trading_pair(&state.pool, id, req).await?;
    Ok(Json(pair))
}

async fn delete_pair(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, ApiError> {
    db::settings::trading_pairs::delete(&state.pool, id).await?;
    Ok(Json(serde_json::json!({"status": "deleted", "id": id })))
}

// --- Settings ---

async fn list_settings(
    State(state): State<Arc<AppState>>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let settings = db::settings::list_settings(&state.pool).await?;
    let map: std::collections::HashMap<String, Value> = settings
        .into_iter()
        .map(|s| (s.key, s.value))
        .collect();
    Ok(Json(serde_json::json!(map)))
}

async fn set_setting(
    State(state): State<Arc<AppState>>,
    Path(key): Path<String>,
    Json(req): Json<UpdateSettingRequest>,
) -> Result<Json<serde_json::Value>, ApiError> {
    crate::admin_controls::set_setting(&state.pool, &key, &req.value).await?;
    Ok(Json(serde_json::json!({"status": "updated", "key": key })))
}

// --- Futures Admin ---

async fn all_futures_positions(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<FuturesPosition>>, ApiError> {
    let positions = db::futures::find_all(&state.pool).await?;
    Ok(Json(positions))
}

async fn force_close_position(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let position = db::futures::find_by_id(&state.pool, id)
        .await?
        .ok_or_else(|| ApiError::NotFound("Position not found".into()))?;

    let price = state.price_feed.read().await.last
        .ok_or_else(|| ApiError::BadRequest("No price feed".into()))?;
    let pnl = crate::futures::FuturesEngine::calculate_pnl(
        position.entry_price, price, position.quantity, &position.side,
    );

    let closed = db::futures::close(&state.pool, position.id, pnl).await?;

    Ok(Json(serde_json::json!({
        "status": "force_closed",
        "id": closed.id,
        "pnl": pnl,
    })))
}

// --- P2P Admin ---

async fn all_p2p_orders(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<P2pOrder>>, ApiError> {
    let orders = db::p2p::orders::list_all(&state.pool).await?;
    Ok(Json(orders))
}

async fn resolve_dispute(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    Json(req): Json<serde_json::Value>,
) -> Result<Json<P2pOrder>, ApiError> {
    let resolve_to = req["action"].as_str()
        .ok_or_else(|| ApiError::BadRequest("action field required (release or refund)".into()))?;
    let order = crate::p2p::P2pEngine::admin_resolve(&state.pool, id, resolve_to).await?;
    Ok(Json(order))
}
