use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Order {
    pub id: Uuid,
    pub user_id: Uuid,
    pub side: String,
    pub order_type: String,
    pub base_currency: String,
    pub quote_currency: String,
    pub price: Option<Decimal>,
    pub quantity: Decimal,
    pub filled: Decimal,
    pub status: String,
    pub stop_price: Option<Decimal>,
    pub reduce_only: bool,
    pub time_in_force: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct PlaceOrderRequest {
    pub side: String,
    pub order_type: String,
    pub base_currency: String,
    pub quote_currency: String,
    pub price: Option<Decimal>,
    pub quantity: Decimal,
    pub stop_price: Option<Decimal>,
    pub reduce_only: Option<bool>,
    pub time_in_force: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderbookLevel {
    pub price: Decimal,
    pub quantity: Decimal,
}

#[derive(Debug, Clone, Serialize)]
pub struct OrderbookSnapshot {
    pub bids: Vec<OrderbookLevel>,
    pub asks: Vec<OrderbookLevel>,
    pub last_price: Option<Decimal>,
    pub timestamp: DateTime<Utc>,
}
