use serde_json::Value;
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::{Currency, SystemSetting, TradingPair};

#[allow(dead_code)]
pub async fn get_setting(pool: &PgPool, key: &str) -> Result<Option<Value>, sqlx::Error> {
    let row: Option<(Value,)> = sqlx::query_as("SELECT value FROM system_settings WHERE key = $1")
        .bind(key)
        .fetch_optional(pool)
        .await?;
    Ok(row.map(|r| r.0))
}

pub async fn set_setting(pool: &PgPool, key: &str, value: &Value) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO system_settings (key, value) VALUES ($1, $2) ON CONFLICT (key) DO UPDATE SET value = $2, updated_at = NOW()",
    )
    .bind(key)
    .bind(value)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn list_settings(pool: &PgPool) -> Result<Vec<SystemSetting>, sqlx::Error> {
    sqlx::query_as::<_, SystemSetting>(
        "SELECT key, value, updated_at FROM system_settings ORDER BY key",
    )
    .fetch_all(pool)
    .await
}

pub mod currencies {
    use super::*;

    pub async fn create(pool: &PgPool, c: &Currency) -> Result<Currency, sqlx::Error> {
        sqlx::query_as::<_, Currency>(
            r#"INSERT INTO currencies (code, name, name_ar, type, decimals, withdrawal_fee, min_withdrawal)
               VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING *"#,
        )
        .bind(&c.code)
        .bind(&c.name)
        .bind(&c.name_ar)
        .bind(&c.r#type)
        .bind(&c.decimals)
        .bind(&c.withdrawal_fee)
        .bind(&c.min_withdrawal)
        .fetch_one(pool)
        .await
    }

    pub async fn list_all(pool: &PgPool) -> Result<Vec<Currency>, sqlx::Error> {
        sqlx::query_as::<_, Currency>(
            "SELECT * FROM currencies ORDER BY code",
        )
        .fetch_all(pool)
        .await
    }

    pub async fn find_by_code(pool: &PgPool, code: &str) -> Result<Option<Currency>, sqlx::Error> {
        sqlx::query_as::<_, Currency>("SELECT * FROM currencies WHERE code = $1")
            .bind(code)
            .fetch_optional(pool)
            .await
    }

    pub async fn update(pool: &PgPool, c: &Currency) -> Result<Currency, sqlx::Error> {
        sqlx::query_as::<_, Currency>(
            r#"UPDATE currencies SET name=$2, name_ar=$3, type=$4, decimals=$5, withdrawal_fee=$6, min_withdrawal=$7, deposit_enabled=$8, withdrawal_enabled=$9, updated_at=NOW()
               WHERE code=$1 RETURNING *"#,
        )
        .bind(&c.code)
        .bind(&c.name)
        .bind(&c.name_ar)
        .bind(&c.r#type)
        .bind(&c.decimals)
        .bind(&c.withdrawal_fee)
        .bind(&c.min_withdrawal)
        .bind(&c.deposit_enabled)
        .bind(&c.withdrawal_enabled)
        .fetch_one(pool)
        .await
    }

    pub async fn delete(pool: &PgPool, code: &str) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM currencies WHERE code = $1")
            .bind(code)
            .execute(pool)
            .await?;
        Ok(())
    }
}

pub mod trading_pairs {
    use super::*;

    pub async fn create(pool: &PgPool, tp: &TradingPair) -> Result<TradingPair, sqlx::Error> {
        sqlx::query_as::<_, TradingPair>(
            r#"INSERT INTO trading_pairs (base_currency, quote_currency, maker_fee, taker_fee, min_trade, max_leverage, futures_enabled, spot_enabled)
               VALUES ($1, $2, $3, $4, $5, $6, $7, $8) RETURNING *"#,
        )
        .bind(&tp.base_currency)
        .bind(&tp.quote_currency)
        .bind(&tp.maker_fee)
        .bind(&tp.taker_fee)
        .bind(&tp.min_trade)
        .bind(&tp.max_leverage)
        .bind(&tp.futures_enabled)
        .bind(&tp.spot_enabled)
        .fetch_one(pool)
        .await
    }

    pub async fn list_all(pool: &PgPool) -> Result<Vec<TradingPair>, sqlx::Error> {
        sqlx::query_as::<_, TradingPair>(
            "SELECT * FROM trading_pairs ORDER BY base_currency",
        )
        .fetch_all(pool)
        .await
    }

    pub async fn find_by_id(pool: &PgPool, id: Uuid) -> Result<Option<TradingPair>, sqlx::Error> {
        sqlx::query_as::<_, TradingPair>("SELECT * FROM trading_pairs WHERE id = $1")
            .bind(id)
            .fetch_optional(pool)
            .await
    }

    pub async fn find_by_pair(pool: &PgPool, base: &str, quote: &str) -> Result<Option<TradingPair>, sqlx::Error> {
        sqlx::query_as::<_, TradingPair>(
            "SELECT * FROM trading_pairs WHERE base_currency = $1 AND quote_currency = $2",
        )
        .bind(base)
        .bind(quote)
        .fetch_optional(pool)
        .await
    }

    pub async fn update(pool: &PgPool, tp: &TradingPair) -> Result<TradingPair, sqlx::Error> {
        sqlx::query_as::<_, TradingPair>(
            r#"UPDATE trading_pairs SET maker_fee=$3, taker_fee=$4, min_trade=$5, max_leverage=$6, futures_enabled=$7, spot_enabled=$8, is_active=$9, updated_at=NOW()
               WHERE base_currency=$1 AND quote_currency=$2 RETURNING *"#,
        )
        .bind(&tp.base_currency)
        .bind(&tp.quote_currency)
        .bind(&tp.maker_fee)
        .bind(&tp.taker_fee)
        .bind(&tp.min_trade)
        .bind(&tp.max_leverage)
        .bind(&tp.futures_enabled)
        .bind(&tp.spot_enabled)
        .bind(&tp.is_active)
        .fetch_one(pool)
        .await
    }

    pub async fn delete(pool: &PgPool, id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM trading_pairs WHERE id = $1")
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }
}
