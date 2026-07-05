use uuid::Uuid;
use rust_decimal::Decimal;
use sqlx::PgPool;

use crate::models::Trade;

pub async fn insert(
    pool: &PgPool,
    buy_order_id: Uuid,
    sell_order_id: Uuid,
    base: &str,
    quote: &str,
    price: Decimal,
    quantity: Decimal,
    total: Decimal,
    taker_side: &str,
) -> Result<Trade, sqlx::Error> {
    sqlx::query_as::<_, Trade>(
        r#"INSERT INTO trades (buy_order_id, sell_order_id, base_currency, quote_currency, price, quantity, total, taker_side)
           VALUES ($1, $2, $3, $4, $5, $6, $7, $8) RETURNING *"#,
    )
    .bind(buy_order_id)
    .bind(sell_order_id)
    .bind(base)
    .bind(quote)
    .bind(price)
    .bind(quantity)
    .bind(total)
    .bind(taker_side)
    .fetch_one(pool)
    .await
}

pub async fn recent(pool: &PgPool, base: &str, quote: &str, limit: i64) -> Result<Vec<Trade>, sqlx::Error> {
    sqlx::query_as::<_, Trade>(
        "SELECT * FROM trades WHERE base_currency = $1 AND quote_currency = $2 ORDER BY created_at DESC LIMIT $3",
    )
    .bind(base)
    .bind(quote)
    .bind(limit)
    .fetch_all(pool)
    .await
}
