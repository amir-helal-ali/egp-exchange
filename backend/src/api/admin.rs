use std::sync::Arc;

use axum::{
    Json, Router, extract::{Path, State, Extension}, middleware, routing::{get, post},
};
use uuid::Uuid;

use crate::{
    AppState, db, errors::ApiError, middleware::auth::{require_auth, require_admin},
    models::*,
    redis::queue::{self, QueueItem},
};

pub fn routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/admin/transactions/pending", get(pending_transactions))
        .route("/admin/transactions/{id}", post(process_transaction))
        .route("/admin/users", get(list_users))
        .route("/admin/wallets", get(all_wallets))
        .route("/admin/liquidity", get(liquidity))
        .route_layer(middleware::from_fn(require_auth))
        .route_layer(middleware::from_fn(require_admin))
}

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
    Json(req): Json<crate::models::transaction::ProcessTransactionRequest>,
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
                &state.pool,
                id,
                admin_id,
                "approved",
                req.notes.as_deref(),
            )
            .await?;

            match tx.tx_type.as_str() {
                "deposit" => {
                    let _ = db::wallets::ensure_wallet(&state.pool, tx.user_id, &tx.currency).await?;
                    let _ = db::wallets::add_balance(
                        &state.pool,
                        tx.user_id,
                        &tx.currency,
                        tx.amount,
                    )
                    .await;
                }
                "withdrawal" => {
                    let _ = db::wallets::add_balance(
                        &state.pool,
                        tx.user_id,
                        &tx.currency,
                        -tx.amount,
                    )
                    .await;
                }
                _ => {}
            }

            let mut redis = state.redis.clone();
            match tx.tx_type.as_str() {
                "deposit" => queue::remove_deposit(&mut redis, tx.id).await?,
                "withdrawal" => queue::remove_withdrawal(&mut redis, tx.id).await?,
                _ => {}
            }

            Ok(Json(processed))
        }
        "reject" => {
            let processed = db::manual_transactions::process(
                &state.pool,
                id,
                admin_id,
                "rejected",
                req.notes.as_deref(),
            )
            .await?;

            let mut redis = state.redis.clone();
            match tx.tx_type.as_str() {
                "deposit" => queue::remove_deposit(&mut redis, tx.id).await?,
                "withdrawal" => queue::remove_withdrawal(&mut redis, tx.id).await?,
                _ => {}
            }

            Ok(Json(processed))
        }
        _ => Err(ApiError::BadRequest("Action must be 'approve' or 'reject'".into())),
    }
}

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
