use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct P2pOffer {
    pub id: Uuid,
    pub user_id: Uuid,
    pub r#type: String,
    pub currency: String,
    pub fiat_currency: String,
    pub price: Decimal,
    pub min_amount: Decimal,
    pub max_amount: Decimal,
    pub available: Decimal,
    pub payment_method: String,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct P2pOrder {
    pub id: Uuid,
    pub offer_id: Uuid,
    pub buyer_id: Uuid,
    pub seller_id: Uuid,
    pub amount: Decimal,
    pub total: Decimal,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct P2pMessage {
    pub id: Uuid,
    pub order_id: Uuid,
    pub sender_id: Uuid,
    pub message: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateOfferRequest {
    pub r#type: String,
    pub currency: String,
    pub price: Decimal,
    pub min_amount: Decimal,
    pub max_amount: Decimal,
    pub available: Decimal,
    pub payment_method: String,
}

#[derive(Debug, Deserialize)]
pub struct AcceptOfferRequest {
    pub amount: Decimal,
}

#[derive(Debug, Deserialize)]
pub struct SendMessageRequest {
    pub message: String,
}
