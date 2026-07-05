use std::sync::Arc;

use axum::{
    Json, Router, extract::{Path, State, Extension}, middleware, routing::{get, post, delete},
};
use uuid::Uuid;

use crate::{
    AppState, db, errors::ApiError, middleware::auth::require_auth,
    models::{Order, OrderbookLevel, OrderbookSnapshot, PlaceOrderRequest},
    engine::orderbook::Order as EngineOrder,
};

pub fn routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/orders", post(place_order))
        .route("/orders", get(list_orders))
        .route("/orders/{id}", delete(cancel_order))
        .route("/orderbook/{base}/{quote}", get(orderbook))
        .route("/trades/{base}/{quote}", get(recent_trades))
        .route_layer(middleware::from_fn(require_auth))
}

async fn place_order(
    State(state): State<Arc<AppState>>,
    Extension(user_id): Extension<Uuid>,
    Json(req): Json<PlaceOrderRequest>,
) -> Result<Json<Order>, ApiError> {
    if req.quantity <= rust_decimal::Decimal::ZERO {
        return Err(ApiError::BadRequest("Quantity must be positive".into()));
    }
    if req.order_type == "limit" && req.price.is_none() {
        return Err(ApiError::BadRequest("Price required for limit orders".into()));
    }

    let base_currency = req.base_currency.to_uppercase();
    let quote_currency = req.quote_currency.to_uppercase();

    let (lock_currency, lock_amount) = match req.side.as_str() {
        "buy" => (quote_currency.clone(), req.price.unwrap_or_default() * req.quantity),
        "sell" => (base_currency.clone(), req.quantity),
        _ => return Err(ApiError::BadRequest("Invalid side".into())),
    };

    let _wallet = db::wallets::lock_balance(&state.pool, user_id, &lock_currency, lock_amount)
        .await
        .map_err(|_| ApiError::BadRequest("Insufficient balance".into()))?;

    let order = db::orders::create(
        &state.pool,
        user_id,
        &req.side,
        &req.order_type,
        &base_currency,
        &quote_currency,
        req.price,
        req.quantity,
    )
    .await?;

    let engine_order = EngineOrder {
        id: order.id,
        user_id,
        side: req.side.clone(),
        price: req.price,
        quantity: req.quantity,
        filled: rust_decimal::Decimal::ZERO,
    };

    let pair_key = (base_currency.clone(), quote_currency.clone());
    let mut engines = state.engines.lock().await;
    let engine = engines
        .entry(pair_key)
        .or_insert_with(|| crate::engine::MatchingEngine::new(base_currency.clone(), quote_currency.clone()));

    let (trades, _result) = match req.order_type.as_str() {
        "market" => engine.process_market(&engine_order),
        _ => engine.process_limit(&engine_order),
    };

    drop(engines);

    for trade in &trades {
        db::trades::insert(
            &state.pool,
            trade.buy_order_id,
            trade.sell_order_id,
            &trade.base_currency,
            &trade.quote_currency,
            trade.price,
            trade.quantity,
            trade.total,
            &trade.taker_side,
        )
        .await?;

        let _ = db::wallets::settle_lock(
            &state.pool,
            trade.buy_user_id,
            &quote_currency,
            trade.total,
        )
        .await;

        let _ = db::wallets::ensure_wallet(&state.pool, trade.buy_user_id, &base_currency).await;
        let _ = db::wallets::add_balance(
            &state.pool,
            trade.buy_user_id,
            &base_currency,
            trade.quantity,
        )
        .await;

        let _ = db::wallets::ensure_wallet(&state.pool, trade.sell_user_id, &quote_currency).await;
        let _ = db::wallets::add_balance(
            &state.pool,
            trade.sell_user_id,
            &quote_currency,
            trade.total,
        )
        .await;

        let _ = db::wallets::settle_lock(
            &state.pool,
            trade.sell_user_id,
            &base_currency,
            trade.quantity,
        )
        .await;
    }

    let filled_qty = trades.iter().map(|t| t.quantity).sum::<rust_decimal::Decimal>();
    let new_status = if filled_qty >= req.quantity {
        "filled"
    } else if filled_qty > rust_decimal::Decimal::ZERO {
        "partial"
    } else {
        "open"
    };

    let updated = db::orders::update_filled(&state.pool, order.id, filled_qty, new_status).await?;

    if filled_qty < req.quantity {
        let unlock_qty = match req.side.as_str() {
            "buy" => {
                let unit_price = req.price.unwrap_or(rust_decimal::Decimal::ZERO);
                unit_price * (req.quantity - filled_qty)
            }
            "sell" => req.quantity - filled_qty,
            _ => rust_decimal::Decimal::ZERO,
        };
        if unlock_qty > rust_decimal::Decimal::ZERO {
            let _ = db::wallets::unlock_balance(
                &state.pool,
                user_id,
                &lock_currency,
                unlock_qty,
            )
            .await;
        }
    }

    Ok(Json(updated))
}

async fn list_orders(
    State(state): State<Arc<AppState>>,
    Extension(user_id): Extension<Uuid>,
) -> Result<Json<Vec<Order>>, ApiError> {
    let orders = db::orders::find_by_user(&state.pool, user_id).await?;
    Ok(Json(orders))
}

async fn cancel_order(
    State(state): State<Arc<AppState>>,
    Extension(user_id): Extension<Uuid>,
    Path(id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let order = db::orders::cancel_order(&state.pool, id, user_id)
        .await?
        .ok_or_else(|| ApiError::NotFound("Order not found or already filled/cancelled".into()))?;

    let unlock_currency = match order.side.as_str() {
        "buy" => order.quote_currency.clone(),
        "sell" => order.base_currency.clone(),
        _ => return Err(ApiError::BadRequest("Invalid side".into())),
    };
    let remaining = order.quantity - order.filled;
    let unlock_amount = match order.side.as_str() {
        "buy" => order.price.unwrap_or_default() * remaining,
        "sell" => remaining,
        _ => rust_decimal::Decimal::ZERO,
    };

    if unlock_amount > rust_decimal::Decimal::ZERO {
        let _ = db::wallets::unlock_balance(
            &state.pool,
            user_id,
            &unlock_currency,
            unlock_amount,
        )
        .await;
    }

    if let Some(price) = order.price {
        let pair_key = (order.base_currency.clone(), order.quote_currency.clone());
        let mut engines = state.engines.lock().await;
        if let Some(engine) = engines.get_mut(&pair_key) {
            engine.cancel_order(&order.side, &price, remaining);
        }
    }

    Ok(Json(serde_json::json!({ "status": "cancelled", "id": order.id })))
}

async fn orderbook(
    State(state): State<Arc<AppState>>,
    Path((base, quote)): Path<(String, String)>,
) -> Result<Json<OrderbookSnapshot>, ApiError> {
    let pair_key = (base.to_uppercase(), quote.to_uppercase());
    let engines = state.engines.lock().await;

    let snapshot = if let Some(engine) = engines.get(&pair_key) {
        let s = engine.orderbook.snapshot(20);
        let last_price = state.price_feed.read().await.last;
        OrderbookSnapshot {
            bids: s.bids.iter().map(|l| OrderbookLevel { price: l.price, quantity: l.quantity }).collect(),
            asks: s.asks.iter().map(|l| OrderbookLevel { price: l.price, quantity: l.quantity }).collect(),
            last_price,
            timestamp: chrono::Utc::now(),
        }
    } else {
        OrderbookSnapshot {
            bids: vec![],
            asks: vec![],
            last_price: None,
            timestamp: chrono::Utc::now(),
        }
    };

    Ok(Json(snapshot))
}

async fn recent_trades(
    State(state): State<Arc<AppState>>,
    Path((base, quote)): Path<(String, String)>,
) -> Result<Json<Vec<crate::models::Trade>>, ApiError> {
    let trades = db::trades::recent(&state.pool, &base.to_uppercase(), &quote.to_uppercase(), 50).await?;
    Ok(Json(trades))
}
