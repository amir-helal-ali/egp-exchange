use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct Wallet {
    pub id: Uuid,
    pub user_id: Uuid,
    pub currency: String,
    pub balance: Decimal,
    pub locked: Decimal,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Wallet {
    pub fn available(&self) -> Decimal {
        self.balance - self.locked
    }
}
