use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct SystemSetting {
    pub key: String,
    pub value: Value,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Currency {
    pub code: String,
    pub name: String,
    pub name_ar: String,
    pub r#type: String,
    pub decimals: i32,
    pub withdrawal_fee: Decimal,
    pub min_withdrawal: Decimal,
    pub deposit_enabled: bool,
    pub withdrawal_enabled: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct TradingPair {
    pub id: Uuid,
    pub base_currency: String,
    pub quote_currency: String,
    pub maker_fee: Decimal,
    pub taker_fee: Decimal,
    pub min_trade: Decimal,
    pub max_leverage: i32,
    pub futures_enabled: bool,
    pub spot_enabled: bool,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateCurrencyRequest {
    pub code: String,
    pub name: String,
    pub name_ar: String,
    pub r#type: String,
    pub decimals: i32,
    pub withdrawal_fee: Decimal,
    pub min_withdrawal: Decimal,
}

#[derive(Debug, Deserialize)]
pub struct CreateTradingPairRequest {
    pub base_currency: String,
    pub quote_currency: String,
    pub maker_fee: Option<Decimal>,
    pub taker_fee: Option<Decimal>,
    pub min_trade: Option<Decimal>,
    pub max_leverage: Option<i32>,
    pub futures_enabled: Option<bool>,
    pub spot_enabled: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateSettingRequest {
    pub value: Value,
}
