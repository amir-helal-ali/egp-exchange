use std::sync::Arc;

use axum::{
    Json, Router, extract::{State, Extension}, middleware, routing::{get, post},
};
use rust_decimal::Decimal;
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    AppState, db, errors::ApiError, middleware::auth::require_auth,
    models::{ManualTransaction, Wallet},
    redis::queue::{QueueItem, push_deposit, push_withdrawal},
};

#[derive(Deserialize)]
pub struct DepositRequest {
    pub amount: Decimal,
}

#[derive(Deserialize)]
pub struct WithdrawRequest {
    pub currency: String,
    pub amount: Decimal,
}

pub fn routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/wallets", get(list_wallets))
        .route("/wallets/transactions", get(list_transactions))
        .route("/wallets/deposit/egp", post(request_deposit))
        .route("/wallets/withdraw", post(request_withdrawal))
        .route_layer(middleware::from_fn(require_auth))
}

async fn list_wallets(
    State(state): State<Arc<AppState>>,
    Extension(user_id): Extension<Uuid>,
) -> Result<Json<Vec<Wallet>>, ApiError> {
    let wallets = db::wallets::find_by_user(&state.pool, user_id).await?;
    Ok(Json(wallets))
}

async fn list_transactions(
    State(state): State<Arc<AppState>>,
    Extension(user_id): Extension<Uuid>,
) -> Result<Json<Vec<ManualTransaction>>, ApiError> {
    let txs = db::manual_transactions::find_by_user(&state.pool, user_id).await?;
    Ok(Json(txs))
}

async fn request_deposit(
    State(state): State<Arc<AppState>>,
    Extension(user_id): Extension<Uuid>,
    Json(req): Json<DepositRequest>,
) -> Result<Json<serde_json::Value>, ApiError> {
    if req.amount <= Decimal::ZERO {
        return Err(ApiError::BadRequest("Amount must be positive".into()));
    }

    let user = db::users::find_by_id(&state.pool, user_id)
        .await?
        .ok_or_else(|| ApiError::NotFound("User not found".into()))?;

    let tx = db::manual_transactions::create(&state.pool, user_id, "deposit", "EGP", req.amount).await?;

    let item = QueueItem {
        tx_id: tx.id,
        user_id,
        user_email: user.email,
        amount: req.amount.to_string(),
        currency: "EGP".into(),
        created_at: tx.created_at.to_rfc3339(),
    };

    let mut redis = state.redis.clone();
    push_deposit(&mut redis, &item).await?;

    Ok(Json(serde_json::json!({
        "id": tx.id,
        "status": "pending",
        "message": "Deposit request submitted for admin approval"
    })))
}

async fn request_withdrawal(
    State(state): State<Arc<AppState>>,
    Extension(user_id): Extension<Uuid>,
    Json(req): Json<WithdrawRequest>,
) -> Result<Json<serde_json::Value>, ApiError> {
    if req.amount <= Decimal::ZERO {
        return Err(ApiError::BadRequest("Amount must be positive".into()));
    }

    let wallet = db::wallets::find_by_user_and_currency(&state.pool, user_id, &req.currency)
        .await?
        .ok_or_else(|| ApiError::BadRequest("Wallet not found".into()))?;

    if wallet.available() < req.amount {
        return Err(ApiError::BadRequest("Insufficient balance".into()));
    }

    let user = db::users::find_by_id(&state.pool, user_id)
        .await?
        .ok_or_else(|| ApiError::NotFound("User not found".into()))?;

    let tx = db::manual_transactions::create(
        &state.pool,
        user_id,
        "withdrawal",
        &req.currency,
        req.amount,
    )
    .await?;

    let item = QueueItem {
        tx_id: tx.id,
        user_id,
        user_email: user.email,
        amount: req.amount.to_string(),
        currency: req.currency,
        created_at: tx.created_at.to_rfc3339(),
    };

    let mut redis = state.redis.clone();
    push_withdrawal(&mut redis, &item).await?;

    Ok(Json(serde_json::json!({
        "id": tx.id,
        "status": "pending",
        "message": "Withdrawal request submitted for admin approval"
    })))
}
