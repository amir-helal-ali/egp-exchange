use uuid::Uuid;
use rust_decimal::Decimal;
use sqlx::PgPool;

use crate::models::Order;

pub async fn create(
    pool: &PgPool,
    user_id: Uuid,
    side: &str,
    order_type: &str,
    base: &str,
    quote: &str,
    price: Option<Decimal>,
    quantity: Decimal,
    stop_price: Option<Decimal>,
    reduce_only: bool,
    time_in_force: &str,
) -> Result<Order, sqlx::Error> {
    sqlx::query_as::<_, Order>(
        r#"INSERT INTO orders (user_id, side, order_type, base_currency, quote_currency, price, quantity, stop_price, reduce_only, time_in_force)
           VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10) RETURNING *"#,
    )
    .bind(user_id)
    .bind(side)
    .bind(order_type)
    .bind(base)
    .bind(quote)
    .bind(price)
    .bind(quantity)
    .bind(stop_price)
    .bind(reduce_only)
    .bind(time_in_force)
    .fetch_one(pool)
    .await
}

pub async fn find_stop_orders(
    pool: &PgPool,
    base: &str,
    quote: &str,
) -> Result<Vec<Order>, sqlx::Error> {
    sqlx::query_as::<_, Order>(
        "SELECT * FROM orders WHERE base_currency = $1 AND quote_currency = $2 AND order_type = 'stop-limit' AND status = 'open' ORDER BY created_at ASC",
    )
    .bind(base)
    .bind(quote)
    .fetch_all(pool)
    .await
}

pub async fn update_trigger(pool: &PgPool, id: Uuid) -> Result<Order, sqlx::Error> {
    sqlx::query_as::<_, Order>(
        "UPDATE orders SET order_type = 'limit', updated_at = NOW() WHERE id = $1 RETURNING *",
    )
    .bind(id)
    .fetch_one(pool)
    .await
}

pub async fn find_by_id(pool: &PgPool, id: Uuid) -> Result<Option<Order>, sqlx::Error> {
    sqlx::query_as::<_, Order>("SELECT * FROM orders WHERE id = $1")
        .bind(id)
        .fetch_optional(pool)
        .await
}

#[allow(dead_code)]
pub async fn find_open_orders(
    pool: &PgPool,
    base: &str,
    quote: &str,
) -> Result<Vec<Order>, sqlx::Error> {
    sqlx::query_as::<_, Order>(
        "SELECT * FROM orders WHERE base_currency = $1 AND quote_currency = $2 AND status IN ('open', 'partial') ORDER BY created_at ASC",
    )
    .bind(base)
    .bind(quote)
    .fetch_all(pool)
    .await
}

pub async fn find_by_user(pool: &PgPool, user_id: Uuid) -> Result<Vec<Order>, sqlx::Error> {
    sqlx::query_as::<_, Order>(
        "SELECT * FROM orders WHERE user_id = $1 ORDER BY created_at DESC",
    )
    .bind(user_id)
    .fetch_all(pool)
    .await
}

pub async fn update_filled(
    pool: &PgPool,
    id: Uuid,
    filled: Decimal,
    status: &str,
) -> Result<Order, sqlx::Error> {
    sqlx::query_as::<_, Order>(
        "UPDATE orders SET filled = $2, status = $3, updated_at = NOW() WHERE id = $1 RETURNING *",
    )
    .bind(id)
    .bind(filled)
    .bind(status)
    .fetch_one(pool)
    .await
}

pub async fn cancel_order(pool: &PgPool, id: Uuid, user_id: Uuid) -> Result<Option<Order>, sqlx::Error> {
    sqlx::query_as::<_, Order>(
        "UPDATE orders SET status = 'cancelled', updated_at = NOW() WHERE id = $1 AND user_id = $2 AND status IN ('open', 'partial') RETURNING *",
    )
    .bind(id)
    .bind(user_id)
    .fetch_optional(pool)
    .await
}
