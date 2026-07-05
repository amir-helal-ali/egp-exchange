use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ManualTransaction {
    pub id: Uuid,
    pub user_id: Uuid,
    pub tx_type: String,
    pub currency: String,
    pub amount: Decimal,
    pub status: String,
    pub admin_id: Option<Uuid>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[allow(dead_code)]
#[derive(Debug, Serialize)]
pub struct TransactionStatus {
    pub id: Uuid,
    pub tx_type: String,
    pub amount: Decimal,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub position: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct ProcessTransactionRequest {
    pub action: String,
    pub notes: Option<String>,
}
