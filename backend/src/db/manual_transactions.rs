use uuid::Uuid;
use rust_decimal::Decimal;
use sqlx::PgPool;

use crate::models::ManualTransaction;

pub async fn create(
    pool: &PgPool,
    user_id: Uuid,
    tx_type: &str,
    currency: &str,
    amount: Decimal,
) -> Result<ManualTransaction, sqlx::Error> {
    sqlx::query_as::<_, ManualTransaction>(
        r#"INSERT INTO manual_transactions (user_id, tx_type, currency, amount)
           VALUES ($1, $2, $3, $4) RETURNING *"#,
    )
    .bind(user_id)
    .bind(tx_type)
    .bind(currency)
    .bind(amount)
    .fetch_one(pool)
    .await
}

pub async fn find_pending(pool: &PgPool) -> Result<Vec<ManualTransaction>, sqlx::Error> {
    sqlx::query_as::<_, ManualTransaction>(
        "SELECT * FROM manual_transactions WHERE status = 'pending' ORDER BY created_at ASC",
    )
    .fetch_all(pool)
    .await
}

pub async fn find_by_user(
    pool: &PgPool,
    user_id: Uuid,
) -> Result<Vec<ManualTransaction>, sqlx::Error> {
    sqlx::query_as::<_, ManualTransaction>(
        "SELECT * FROM manual_transactions WHERE user_id = $1 ORDER BY created_at DESC",
    )
    .bind(user_id)
    .fetch_all(pool)
    .await
}

pub async fn find_by_id(
    pool: &PgPool,
    id: Uuid,
) -> Result<Option<ManualTransaction>, sqlx::Error> {
    sqlx::query_as::<_, ManualTransaction>(
        "SELECT * FROM manual_transactions WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(pool)
    .await
}

pub async fn process(
    pool: &PgPool,
    id: Uuid,
    admin_id: Uuid,
    status: &str,
    notes: Option<&str>,
) -> Result<ManualTransaction, sqlx::Error> {
    sqlx::query_as::<_, ManualTransaction>(
        "UPDATE manual_transactions SET status = $2, admin_id = $3, notes = $4, updated_at = NOW() WHERE id = $1 RETURNING *",
    )
    .bind(id)
    .bind(status)
    .bind(admin_id)
    .bind(notes)
    .fetch_one(pool)
    .await
}
