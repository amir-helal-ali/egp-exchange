use rust_decimal::Decimal;
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::FuturesPosition;

pub async fn create(pool: &PgPool, pos: &FuturesPosition) -> Result<FuturesPosition, sqlx::Error> {
    sqlx::query_as::<_, FuturesPosition>(
        r#"INSERT INTO futures_positions (user_id, pair, side, quantity, entry_price, mark_price, liquidation_price, leverage, margin)
           VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9) RETURNING *"#,
    )
    .bind(&pos.user_id)
    .bind(&pos.pair)
    .bind(&pos.side)
    .bind(&pos.quantity)
    .bind(&pos.entry_price)
    .bind(&pos.mark_price)
    .bind(&pos.liquidation_price)
    .bind(&pos.leverage)
    .bind(&pos.margin)
    .fetch_one(pool)
    .await
}

pub async fn update_tp_sl(
    pool: &PgPool,
    id: Uuid,
    user_id: Uuid,
    take_profit: Option<Decimal>,
    stop_loss: Option<Decimal>,
) -> Result<Option<FuturesPosition>, sqlx::Error> {
    sqlx::query_as::<_, FuturesPosition>(
        "UPDATE futures_positions SET take_profit = $3, stop_loss = $4, updated_at = NOW() WHERE id = $1 AND user_id = $2 AND status = 'open' RETURNING *",
    )
    .bind(id)
    .bind(user_id)
    .bind(take_profit)
    .bind(stop_loss)
    .fetch_optional(pool)
    .await
}

pub async fn partial_close(
    pool: &PgPool,
    id: Uuid,
    reduce_qty: Decimal,
    pnl: Decimal,
) -> Result<Option<FuturesPosition>, sqlx::Error> {
    sqlx::query_as::<_, FuturesPosition>(
        r#"
        WITH updated AS (
            UPDATE futures_positions
            SET quantity = quantity - $2,
                realized_pnl = realized_pnl + $3,
                margin = margin * (quantity - $2) / quantity,
                updated_at = NOW()
            WHERE id = $1 AND status = 'open' AND quantity > $2
            RETURNING *
        )
        SELECT * FROM updated
        "#,
    )
    .bind(id)
    .bind(reduce_qty)
    .bind(pnl)
    .fetch_optional(pool)
    .await
}

pub async fn find_by_tp_sl_triggered(
    pool: &PgPool,
    mark_price: Decimal,
) -> Result<Vec<FuturesPosition>, sqlx::Error> {
    sqlx::query_as::<_, FuturesPosition>(
        r#"
        SELECT * FROM futures_positions
        WHERE status = 'open'
        AND (
            (side = 'long' AND stop_loss IS NOT NULL AND stop_loss >= $1)
            OR (side = 'long' AND take_profit IS NOT NULL AND take_profit <= $1)
            OR (side = 'short' AND stop_loss IS NOT NULL AND stop_loss <= $1)
            OR (side = 'short' AND take_profit IS NOT NULL AND take_profit >= $1)
        )
        "#,
    )
    .bind(mark_price)
    .fetch_all(pool)
    .await
}

pub async fn find_by_user(pool: &PgPool, user_id: Uuid) -> Result<Vec<FuturesPosition>, sqlx::Error> {
    sqlx::query_as::<_, FuturesPosition>(
        "SELECT * FROM futures_positions WHERE user_id = $1 AND status = 'open' ORDER BY created_at DESC",
    )
    .bind(user_id)
    .fetch_all(pool)
    .await
}

pub async fn find_all_open(pool: &PgPool) -> Result<Vec<FuturesPosition>, sqlx::Error> {
    sqlx::query_as::<_, FuturesPosition>(
        "SELECT * FROM futures_positions WHERE status = 'open'",
    )
    .fetch_all(pool)
    .await
}

pub async fn find_by_id(pool: &PgPool, id: Uuid) -> Result<Option<FuturesPosition>, sqlx::Error> {
    sqlx::query_as::<_, FuturesPosition>("SELECT * FROM futures_positions WHERE id = $1")
        .bind(id)
        .fetch_optional(pool)
        .await
}

pub async fn update_price(
    pool: &PgPool,
    id: Uuid,
    mark_price: Decimal,
    unrealized_pnl: Decimal,
) -> Result<FuturesPosition, sqlx::Error> {
    sqlx::query_as::<_, FuturesPosition>(
        "UPDATE futures_positions SET mark_price = $2, unrealized_pnl = $3, updated_at = NOW() WHERE id = $1 RETURNING *",
    )
    .bind(id)
    .bind(mark_price)
    .bind(unrealized_pnl)
    .fetch_one(pool)
    .await
}

pub async fn close(pool: &PgPool, id: Uuid, pnl: Decimal) -> Result<FuturesPosition, sqlx::Error> {
    sqlx::query_as::<_, FuturesPosition>(
        "UPDATE futures_positions SET status = 'closed', realized_pnl = $2, updated_at = NOW() WHERE id = $1 RETURNING *",
    )
    .bind(id)
    .bind(pnl)
    .fetch_one(pool)
    .await
}

pub async fn find_all(pool: &PgPool) -> Result<Vec<FuturesPosition>, sqlx::Error> {
    sqlx::query_as::<_, FuturesPosition>(
        "SELECT * FROM futures_positions ORDER BY created_at DESC",
    )
    .fetch_all(pool)
    .await
}
