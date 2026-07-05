use std::sync::Arc;

use axum::{
    Extension, Json, Router,
    extract::{Path, State},
    middleware,
    routing::{delete, get, post},
};
use uuid::Uuid;

use crate::{
    AppState, db, errors::ApiError,
    futures::FuturesEngine,
    middleware::auth::require_auth,
    models::{FuturesPosition, OpenPositionRequest, SetTpSlRequest, PartialCloseRequest},
};

pub fn routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/futures/positions", post(open_position))
        .route("/futures/positions", get(list_positions))
        .route("/futures/positions/:id", get(get_position))
        .route("/futures/positions/:id", delete(close_position))
        .route("/futures/positions/:id/tp-sl", post(set_tp_sl))
        .route("/futures/positions/:id/partial-close", post(partial_close))
        .route_layer(middleware::from_fn(require_auth))
}

async fn open_position(
    State(state): State<Arc<AppState>>,
    Extension(user_id): Extension<Uuid>,
    Json(req): Json<OpenPositionRequest>,
) -> Result<Json<FuturesPosition>, ApiError> {
    let pair_upper = req.pair.to_uppercase();
    let parts: Vec<&str> = pair_upper.split(['/', '-']).collect();
    if parts.len() != 2 {
        return Err(ApiError::BadRequest("Pair format must be BASE/QUOTE".into()));
    }
    let (base, quote) = (parts[0], parts[1]);

    let pair_info = db::settings::trading_pairs::find_by_pair(&state.pool, base, quote)
        .await?
        .ok_or_else(|| ApiError::BadRequest("Trading pair not found".into()))?;

    if !pair_info.futures_enabled {
        return Err(ApiError::BadRequest("Futures not enabled for this pair".into()));
    }
    if req.leverage > pair_info.max_leverage {
        return Err(ApiError::BadRequest("Leverage exceeds maximum allowed".into()));
    }
    if req.quantity < pair_info.min_trade {
        return Err(ApiError::BadRequest("Quantity below minimum trade size".into()));
    }

    let price = state.price_feed.read().await.last
        .ok_or_else(|| ApiError::BadRequest("No price feed available".into()))?;

    let margin = FuturesEngine::calculate_margin(req.quantity, price, req.leverage);
    let _ = db::wallets::lock_balance(&state.pool, user_id, quote, margin)
        .await
        .map_err(|_| ApiError::BadRequest("Insufficient balance for margin".into()))?;

    let position = FuturesEngine::open_position(
        &state.pool,
        user_id,
        &pair_upper,
        &req.side,
        req.quantity,
        req.leverage,
        price,
    )
    .await?;

    // Publish position update
    let _ = state.ws_pubsub.publish_json(&format!("positions:{}", user_id), &serde_json::json!({
        "channel": format!("positions:{}", user_id),
        "data": &position,
    })).await;

    Ok(Json(position))
}

async fn list_positions(
    State(state): State<Arc<AppState>>,
    Extension(user_id): Extension<Uuid>,
) -> Result<Json<Vec<FuturesPosition>>, ApiError> {
    let positions = db::futures::find_by_user(&state.pool, user_id).await?;
    Ok(Json(positions))
}

async fn get_position(
    State(state): State<Arc<AppState>>,
    Extension(_user_id): Extension<Uuid>,
    Path(id): Path<Uuid>,
) -> Result<Json<FuturesPosition>, ApiError> {
    let position = db::futures::find_by_id(&state.pool, id)
        .await?
        .ok_or_else(|| ApiError::NotFound("Position not found".into()))?;
    Ok(Json(position))
}

async fn close_position(
    State(state): State<Arc<AppState>>,
    Extension(user_id): Extension<Uuid>,
    Path(id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let position = db::futures::find_by_id(&state.pool, id)
        .await?
        .ok_or_else(|| ApiError::NotFound("Position not found".into()))?;

    if position.user_id != user_id {
        return Err(ApiError::Forbidden);
    }
    if position.status != "open" {
        return Err(ApiError::BadRequest("Position already closed".into()));
    }

    let price = state.price_feed.read().await.last
        .ok_or_else(|| ApiError::BadRequest("No price feed".into()))?;
    let pnl = FuturesEngine::calculate_pnl(
        position.entry_price, price, position.quantity, &position.side,
    );

    let closed = db::futures::close(&state.pool, position.id, pnl).await?;

    let quote = position.pair.split(['/', '-']).collect::<Vec<&str>>()[1].to_string();
    // Return margin to available balance + add PnL
    let _ = db::wallets::unlock_balance(&state.pool, user_id, &quote, position.margin).await;
    if pnl > rust_decimal::Decimal::ZERO {
        let _ = db::wallets::add_balance(&state.pool, user_id, &quote, pnl).await;
    }

    // Publish position removal
    let _ = state.ws_pubsub.publish_json(&format!("positions:{}", user_id), &serde_json::json!({
        "channel": format!("positions:{}", user_id),
        "data": serde_json::json!({"action": "closed", "id": closed.id, "pnl": pnl}),
    })).await;

    Ok(Json(serde_json::json!({
        "status": "closed",
        "id": closed.id,
        "pnl": pnl,
    })))
}

async fn set_tp_sl(
    State(state): State<Arc<AppState>>,
    Extension(user_id): Extension<Uuid>,
    Path(id): Path<Uuid>,
    Json(req): Json<SetTpSlRequest>,
) -> Result<Json<FuturesPosition>, ApiError> {
    let updated = db::futures::update_tp_sl(&state.pool, id, user_id, req.take_profit, req.stop_loss)
        .await?
        .ok_or_else(|| ApiError::NotFound("Position not found or already closed".into()))?;

    // Publish position update
    let _ = state.ws_pubsub.publish_json(&format!("positions:{}", user_id), &serde_json::json!({
        "channel": format!("positions:{}", user_id),
        "data": &updated,
    })).await;

    Ok(Json(updated))
}

async fn partial_close(
    State(state): State<Arc<AppState>>,
    Extension(user_id): Extension<Uuid>,
    Path(id): Path<Uuid>,
    Json(req): Json<PartialCloseRequest>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let position = db::futures::find_by_id(&state.pool, id)
        .await?
        .ok_or_else(|| ApiError::NotFound("Position not found".into()))?;

    if position.user_id != user_id {
        return Err(ApiError::Forbidden);
    }
    if position.status != "open" {
        return Err(ApiError::BadRequest("Position already closed".into()));
    }
    if req.quantity <= rust_decimal::Decimal::ZERO || req.quantity >= position.quantity {
        return Err(ApiError::BadRequest("Invalid partial close quantity".into()));
    }

    let price = state.price_feed.read().await.last
        .ok_or_else(|| ApiError::BadRequest("No price feed".into()))?;
    let pnl = FuturesEngine::calculate_pnl(
        position.entry_price, price, req.quantity, &position.side,
    );

    let closed = db::futures::partial_close(&state.pool, position.id, req.quantity, pnl)
        .await?
        .ok_or_else(|| ApiError::BadRequest("Could not close position".into()))?;

    let quote = position.pair.split(['/', '-']).collect::<Vec<&str>>()[1].to_string();
    let margin_ratio = req.quantity / position.quantity;
    let released_margin = position.margin * margin_ratio;
    let _ = db::wallets::unlock_balance(&state.pool, user_id, &quote, released_margin).await;
    if pnl > rust_decimal::Decimal::ZERO {
        let _ = db::wallets::add_balance(&state.pool, user_id, &quote, pnl).await;
    }

    // Publish position update
    let _ = state.ws_pubsub.publish_json(&format!("positions:{}", user_id), &serde_json::json!({
        "channel": format!("positions:{}", user_id),
        "data": serde_json::json!({
            "action": "partial_closed",
            "id": closed.id,
            "remaining_quantity": closed.quantity,
            "closed_quantity": req.quantity,
            "pnl": pnl,
        }),
    })).await;

    Ok(Json(serde_json::json!({
        "status": "partial_closed",
        "remaining_quantity": closed.quantity,
        "closed_quantity": req.quantity,
        "pnl": pnl,
    })))
}
