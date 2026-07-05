pub mod engine;

use rust_decimal::Decimal;
use uuid::Uuid;

use crate::models::{P2pOffer, P2pOrder};

pub struct P2pEngine;

impl P2pEngine {
    pub async fn accept_offer(
        pool: &sqlx::PgPool,
        offer_id: Uuid,
        buyer_id: Uuid,
        amount: Decimal,
    ) -> Result<(P2pOrder, P2pOffer), crate::errors::ApiError> {
        let offer = crate::db::p2p::offers::find_by_id(pool, offer_id)
            .await?
            .ok_or_else(|| crate::errors::ApiError::NotFound("Offer not found".into()))?;

        if offer.status != "active" {
            return Err(crate::errors::ApiError::BadRequest("Offer is not active".into()));
        }
        if offer.user_id == buyer_id {
            return Err(crate::errors::ApiError::BadRequest("Cannot accept your own offer".into()));
        }
        if amount < offer.min_amount || amount > offer.max_amount {
            return Err(crate::errors::ApiError::BadRequest("Amount out of range".into()));
        }
        if amount > offer.available {
            return Err(crate::errors::ApiError::BadRequest("Insufficient available amount".into()));
        }

        let total = offer.price * amount;

        let order = P2pOrder {
            id: uuid::Uuid::new_v4(),
            offer_id,
            buyer_id,
            seller_id: offer.user_id,
            amount,
            total,
            status: "pending".to_string(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };

        let order = crate::db::p2p::orders::create(pool, &order).await?;

        let new_available = offer.available - amount;
        let updated_offer = if new_available <= Decimal::ZERO {
            crate::db::p2p::offers::deactivate(pool, offer_id).await?
        } else {
            crate::db::p2p::offers::update_available(pool, offer_id, new_available).await?
        };

        Ok((order, updated_offer))
    }

    pub async fn confirm_paid(
        pool: &sqlx::PgPool,
        order_id: Uuid,
        user_id: Uuid,
    ) -> Result<P2pOrder, crate::errors::ApiError> {
        let order = crate::db::p2p::orders::find_by_id(pool, order_id)
            .await?
            .ok_or_else(|| crate::errors::ApiError::NotFound("Order not found".into()))?;

        if order.buyer_id != user_id {
            return Err(crate::errors::ApiError::Forbidden);
        }
        if order.status != "pending" {
            return Err(crate::errors::ApiError::BadRequest("Invalid order status".into()));
        }

        crate::db::p2p::orders::update_status(pool, order_id, "paid").await
            .map_err(crate::errors::ApiError::from)
    }

    pub async fn release_escrow(
        pool: &sqlx::PgPool,
        order_id: Uuid,
        user_id: Uuid,
    ) -> Result<P2pOrder, crate::errors::ApiError> {
        let order = crate::db::p2p::orders::find_by_id(pool, order_id)
            .await?
            .ok_or_else(|| crate::errors::ApiError::NotFound("Order not found".into()))?;

        if order.seller_id != user_id {
            return Err(crate::errors::ApiError::Forbidden);
        }
        if order.status != "paid" {
            return Err(crate::errors::ApiError::BadRequest("Buyer has not confirmed payment yet".into()));
        }

        // Credit buyer's wallet
        let offer = crate::db::p2p::offers::find_by_id(pool, order.offer_id)
            .await?
            .ok_or_else(|| crate::errors::ApiError::NotFound("Offer not found".into()))?;

        let _ = crate::db::wallets::ensure_wallet(pool, order.buyer_id, &offer.currency).await;
        let _ = crate::db::wallets::add_balance(pool, order.buyer_id, &offer.currency, order.amount).await;

        crate::db::p2p::orders::update_status(pool, order_id, "completed").await
            .map_err(crate::errors::ApiError::from)
    }

    pub async fn dispute_order(
        pool: &sqlx::PgPool,
        order_id: Uuid,
        user_id: Uuid,
    ) -> Result<P2pOrder, crate::errors::ApiError> {
        let order = crate::db::p2p::orders::find_by_id(pool, order_id)
            .await?
            .ok_or_else(|| crate::errors::ApiError::NotFound("Order not found".into()))?;

        if order.buyer_id != user_id && order.seller_id != user_id {
            return Err(crate::errors::ApiError::Forbidden);
        }

        crate::db::p2p::orders::update_status(pool, order_id, "disputed").await
            .map_err(crate::errors::ApiError::from)
    }

    pub async fn admin_resolve(
        pool: &sqlx::PgPool,
        order_id: Uuid,
        resolve_to: &str,
    ) -> Result<P2pOrder, crate::errors::ApiError> {
        let order = crate::db::p2p::orders::find_by_id(pool, order_id)
            .await?
            .ok_or_else(|| crate::errors::ApiError::NotFound("Order not found".into()))?;

        let new_status = match resolve_to {
            "release" => "completed",
            "refund" => "cancelled",
            _ => return Err(crate::errors::ApiError::BadRequest("Invalid resolution".into())),
        };

        if resolve_to == "release" {
            let offer = crate::db::p2p::offers::find_by_id(pool, order.offer_id)
                .await?
                .ok_or_else(|| crate::errors::ApiError::NotFound("Offer not found".into()))?;
            let _ = crate::db::wallets::ensure_wallet(pool, order.buyer_id, &offer.currency).await;
            let _ = crate::db::wallets::add_balance(pool, order.buyer_id, &offer.currency, order.amount).await;
        }

        crate::db::p2p::orders::update_status(pool, order_id, new_status).await
            .map_err(crate::errors::ApiError::from)
    }
}
