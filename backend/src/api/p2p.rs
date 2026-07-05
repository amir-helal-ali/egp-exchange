use std::sync::Arc;

use axum::{
    Extension, Json, Router,
    extract::{Path, State},
    middleware,
    routing::{get, post},
};
use uuid::Uuid;

use crate::{
    AppState, db, errors::ApiError,
    middleware::auth::require_auth,
    models::*,
    p2p::P2pEngine,
};

pub fn routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/p2p/offers", post(create_offer))
        .route("/p2p/offers", get(list_offers))
        .route("/p2p/offers/:id/accept", post(accept_offer))
        .route("/p2p/orders", get(my_orders))
        .route("/p2p/orders/:id/paid", post(confirm_paid))
        .route("/p2p/orders/:id/release", post(release_escrow))
        .route("/p2p/orders/:id/dispute", post(dispute_order))
        .route("/p2p/orders/:id/messages", post(send_message))
        .route("/p2p/orders/:id/messages", get(get_messages))
        .route_layer(middleware::from_fn(require_auth))
}

async fn create_offer(
    State(state): State<Arc<AppState>>,
    Extension(user_id): Extension<Uuid>,
    Json(req): Json<CreateOfferRequest>,
) -> Result<Json<P2pOffer>, ApiError> {
    let wallet = db::wallets::find_by_user_and_currency(&state.pool, user_id, &req.currency.to_uppercase())
        .await?
        .ok_or_else(|| ApiError::BadRequest("Wallet not found".into()))?;

    if wallet.available() < req.available {
        return Err(ApiError::BadRequest("Insufficient balance".into()));
    }

    let _ = db::wallets::lock_balance(&state.pool, user_id, &req.currency.to_uppercase(), req.available)
        .await
        .map_err(|_| ApiError::BadRequest("Failed to lock balance".into()))?;

    let offer = db::p2p::offers::create(&state.pool, &P2pOffer {
        id: Uuid::new_v4(),
        user_id,
        r#type: req.r#type,
        currency: req.currency.to_uppercase(),
        fiat_currency: "EGP".into(),
        price: req.price,
        min_amount: req.min_amount,
        max_amount: req.max_amount,
        available: req.available,
        payment_method: req.payment_method,
        status: "active".into(),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    }).await?;

    Ok(Json(offer))
}

async fn list_offers(
    State(state): State<Arc<AppState>>,
    axum::extract::Query(params): axum::extract::Query<std::collections::HashMap<String, String>>,
) -> Result<Json<Vec<P2pOffer>>, ApiError> {
    let r#type = params.get("type").map(String::as_str);
    let currency = params.get("currency").map(String::as_str);
    let offers = db::p2p::offers::find_active(&state.pool, r#type, currency).await?;
    Ok(Json(offers))
}

async fn accept_offer(
    State(state): State<Arc<AppState>>,
    Extension(user_id): Extension<Uuid>,
    Path(id): Path<Uuid>,
    Json(req): Json<AcceptOfferRequest>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let (order, offer) = P2pEngine::accept_offer(&state.pool, id, user_id, req.amount).await?;
    Ok(Json(serde_json::json!({
        "order": order,
        "offer": offer,
        "message": "Offer accepted. Please complete payment within the agreed time."
    })))
}

async fn my_orders(
    State(state): State<Arc<AppState>>,
    Extension(user_id): Extension<Uuid>,
) -> Result<Json<Vec<P2pOrder>>, ApiError> {
    let orders = db::p2p::orders::find_by_user(&state.pool, user_id).await?;
    Ok(Json(orders))
}

async fn confirm_paid(
    State(state): State<Arc<AppState>>,
    Extension(user_id): Extension<Uuid>,
    Path(id): Path<Uuid>,
) -> Result<Json<P2pOrder>, ApiError> {
    let order = P2pEngine::confirm_paid(&state.pool, id, user_id).await?;
    Ok(Json(order))
}

async fn release_escrow(
    State(state): State<Arc<AppState>>,
    Extension(user_id): Extension<Uuid>,
    Path(id): Path<Uuid>,
) -> Result<Json<P2pOrder>, ApiError> {
    let order = P2pEngine::release_escrow(&state.pool, id, user_id).await?;
    Ok(Json(order))
}

async fn dispute_order(
    State(state): State<Arc<AppState>>,
    Extension(user_id): Extension<Uuid>,
    Path(id): Path<Uuid>,
) -> Result<Json<P2pOrder>, ApiError> {
    let order = P2pEngine::dispute_order(&state.pool, id, user_id).await?;
    Ok(Json(order))
}

async fn send_message(
    State(state): State<Arc<AppState>>,
    Extension(user_id): Extension<Uuid>,
    Path(id): Path<Uuid>,
    Json(req): Json<SendMessageRequest>,
) -> Result<Json<P2pMessage>, ApiError> {
    let msg = db::p2p::messages::send(&state.pool, id, user_id, &req.message).await?;
    Ok(Json(msg))
}

async fn get_messages(
    State(state): State<Arc<AppState>>,
    Extension(_user_id): Extension<Uuid>,
    Path(id): Path<Uuid>,
) -> Result<Json<Vec<P2pMessage>>, ApiError> {
    let messages = db::p2p::messages::by_order(&state.pool, id).await?;
    Ok(Json(messages))
}
