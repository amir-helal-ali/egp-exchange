use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct FuturesPosition {
    pub id: Uuid,
    pub user_id: Uuid,
    pub pair: String,
    pub side: String,
    pub quantity: Decimal,
    pub entry_price: Decimal,
    pub mark_price: Decimal,
    pub liquidation_price: Decimal,
    pub leverage: i32,
    pub margin: Decimal,
    pub unrealized_pnl: Decimal,
    pub realized_pnl: Decimal,
    pub status: String,
    pub take_profit: Option<Decimal>,
    pub stop_loss: Option<Decimal>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct OpenPositionRequest {
    pub pair: String,
    pub side: String,
    pub quantity: Decimal,
    pub leverage: i32,
}

#[derive(Debug, Deserialize)]
pub struct SetTpSlRequest {
    pub take_profit: Option<Decimal>,
    pub stop_loss: Option<Decimal>,
}

#[derive(Debug, Deserialize)]
pub struct PartialCloseRequest {
    pub quantity: Decimal,
}
