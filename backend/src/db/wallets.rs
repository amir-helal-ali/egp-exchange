use uuid::Uuid;
use rust_decimal::Decimal;
use sqlx::PgPool;

use crate::models::Wallet;

pub async fn find_by_user(pool: &PgPool, user_id: Uuid) -> Result<Vec<Wallet>, sqlx::Error> {
    sqlx::query_as::<_, Wallet>("SELECT * FROM wallets WHERE user_id = $1")
        .bind(user_id)
        .fetch_all(pool)
        .await
}

pub async fn find_by_user_and_currency(
    pool: &PgPool,
    user_id: Uuid,
    currency: &str,
) -> Result<Option<Wallet>, sqlx::Error> {
    sqlx::query_as::<_, Wallet>(
        "SELECT * FROM wallets WHERE user_id = $1 AND currency = $2",
    )
    .bind(user_id)
    .bind(currency)
    .fetch_optional(pool)
    .await
}

pub async fn ensure_wallet(
    pool: &PgPool,
    user_id: Uuid,
    currency: &str,
) -> Result<Wallet, sqlx::Error> {
    let maybe = find_by_user_and_currency(pool, user_id, currency).await?;
    if let Some(w) = maybe {
        return Ok(w);
    }
    sqlx::query_as::<_, Wallet>(
        "INSERT INTO wallets (user_id, currency) VALUES ($1, $2) RETURNING *",
    )
    .bind(user_id)
    .bind(currency)
    .fetch_one(pool)
    .await
}

pub async fn add_balance(
    pool: &PgPool,
    user_id: Uuid,
    currency: &str,
    amount: Decimal,
) -> Result<Wallet, sqlx::Error> {
    sqlx::query_as::<_, Wallet>(
        "UPDATE wallets SET balance = balance + $3, updated_at = NOW() WHERE user_id = $1 AND currency = $2 RETURNING *",
    )
    .bind(user_id)
    .bind(currency)
    .bind(amount)
    .fetch_one(pool)
    .await
}

pub async fn lock_balance(
    pool: &PgPool,
    user_id: Uuid,
    currency: &str,
    amount: Decimal,
) -> Result<Wallet, sqlx::Error> {
    sqlx::query_as::<_, Wallet>(
        "UPDATE wallets SET balance = balance - $3, locked = locked + $3, updated_at = NOW() WHERE user_id = $1 AND currency = $2 AND balance >= $3 RETURNING *",
    )
    .bind(user_id)
    .bind(currency)
    .bind(amount)
    .fetch_one(pool)
    .await
}

pub async fn unlock_balance(
    pool: &PgPool,
    user_id: Uuid,
    currency: &str,
    amount: Decimal,
) -> Result<Wallet, sqlx::Error> {
    sqlx::query_as::<_, Wallet>(
        "UPDATE wallets SET balance = balance + $3, locked = locked - $3, updated_at = NOW() WHERE user_id = $1 AND currency = $2 AND locked >= $3 RETURNING *",
    )
    .bind(user_id)
    .bind(currency)
    .bind(amount)
    .fetch_one(pool)
    .await
}

pub async fn settle_lock(
    pool: &PgPool,
    user_id: Uuid,
    currency: &str,
    amount: Decimal,
) -> Result<Wallet, sqlx::Error> {
    sqlx::query_as::<_, Wallet>(
        "UPDATE wallets SET locked = locked - $3, updated_at = NOW() WHERE user_id = $1 AND currency = $2 AND locked >= $3 RETURNING *",
    )
    .bind(user_id)
    .bind(currency)
    .bind(amount)
    .fetch_one(pool)
    .await
}

pub async fn list_all(pool: &PgPool) -> Result<Vec<Wallet>, sqlx::Error> {
    sqlx::query_as::<_, Wallet>("SELECT * FROM wallets ORDER BY user_id, currency")
        .fetch_all(pool)
        .await
}
