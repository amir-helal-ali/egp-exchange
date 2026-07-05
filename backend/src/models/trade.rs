use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct Trade {
    pub id: Uuid,
    pub buy_order_id: Uuid,
    pub sell_order_id: Uuid,
    pub base_currency: String,
    pub quote_currency: String,
    pub price: Decimal,
    pub quantity: Decimal,
    pub total: Decimal,
    pub taker_side: String,
    pub created_at: DateTime<Utc>,
}
