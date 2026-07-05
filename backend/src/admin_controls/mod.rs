use rust_decimal::Decimal;
use serde_json::Value;
use uuid::Uuid;

use crate::db;
use crate::errors::ApiError;
use crate::models::*;

pub async fn create_currency(
    pool: &sqlx::PgPool,
    req: settings::CreateCurrencyRequest,
) -> Result<Currency, ApiError> {
    let currency = Currency {
        code: req.code.to_uppercase(),
        name: req.name,
        name_ar: req.name_ar,
        r#type: req.r#type,
        decimals: req.decimals,
        withdrawal_fee: req.withdrawal_fee,
        min_withdrawal: req.min_withdrawal,
        deposit_enabled: true,
        withdrawal_enabled: true,
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };
    db::settings::currencies::create(pool, &currency).await.map_err(ApiError::from)
}

pub async fn update_currency(
    pool: &sqlx::PgPool,
    req: settings::CreateCurrencyRequest,
) -> Result<Currency, ApiError> {
    let existing = db::settings::currencies::find_by_code(pool, &req.code.to_uppercase())
        .await?
        .ok_or_else(|| ApiError::NotFound("Currency not found".into()))?;

    let currency = Currency {
        code: req.code.to_uppercase(),
        name: req.name,
        name_ar: req.name_ar,
        r#type: req.r#type,
        decimals: req.decimals,
        withdrawal_fee: req.withdrawal_fee,
        min_withdrawal: req.min_withdrawal,
        deposit_enabled: existing.deposit_enabled,
        withdrawal_enabled: existing.withdrawal_enabled,
        created_at: existing.created_at,
        updated_at: chrono::Utc::now(),
    };
    db::settings::currencies::update(pool, &currency).await.map_err(ApiError::from)
}

pub async fn toggle_currency_feature(
    pool: &sqlx::PgPool,
    code: &str,
    feature: &str,
    enabled: bool,
) -> Result<Currency, ApiError> {
    let mut currency = db::settings::currencies::find_by_code(pool, code)
        .await?
        .ok_or_else(|| ApiError::NotFound("Currency not found".into()))?;

    match feature {
        "deposit" => currency.deposit_enabled = enabled,
        "withdrawal" => currency.withdrawal_enabled = enabled,
        _ => return Err(ApiError::BadRequest("Invalid feature".into())),
    }
    currency.updated_at = chrono::Utc::now();
    db::settings::currencies::update(pool, &currency).await.map_err(ApiError::from)
}

pub async fn create_trading_pair(
    pool: &sqlx::PgPool,
    req: settings::CreateTradingPairRequest,
) -> Result<TradingPair, ApiError> {
    let pair = TradingPair {
        id: Uuid::new_v4(),
        base_currency: req.base_currency.to_uppercase(),
        quote_currency: req.quote_currency.to_uppercase(),
        maker_fee: req.maker_fee.unwrap_or(Decimal::new(1, 3)),
        taker_fee: req.taker_fee.unwrap_or(Decimal::new(1, 3)),
        min_trade: req.min_trade.unwrap_or(Decimal::new(1, 4)),
        max_leverage: req.max_leverage.unwrap_or(1),
        futures_enabled: req.futures_enabled.unwrap_or(false),
        spot_enabled: req.spot_enabled.unwrap_or(true),
        is_active: true,
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };
    db::settings::trading_pairs::create(pool, &pair).await.map_err(ApiError::from)
}

pub async fn update_trading_pair(
    pool: &sqlx::PgPool,
    id: Uuid,
    req: settings::CreateTradingPairRequest,
) -> Result<TradingPair, ApiError> {
    let mut pair = db::settings::trading_pairs::find_by_id(pool, id)
        .await?
        .ok_or_else(|| ApiError::NotFound("Trading pair not found".into()))?;

    if let Some(fee) = req.maker_fee { pair.maker_fee = fee; }
    if let Some(fee) = req.taker_fee { pair.taker_fee = fee; }
    if let Some(mt) = req.min_trade { pair.min_trade = mt; }
    if let Some(ml) = req.max_leverage { pair.max_leverage = ml; }
    if let Some(fe) = req.futures_enabled { pair.futures_enabled = fe; }
    if let Some(se) = req.spot_enabled { pair.spot_enabled = se; }

    db::settings::trading_pairs::update(pool, &pair).await.map_err(ApiError::from)
}

pub async fn set_setting(
    pool: &sqlx::PgPool,
    key: &str,
    value: &Value,
) -> Result<(), ApiError> {
    db::settings::set_setting(pool, key, value)
        .await
        .map_err(ApiError::from)
}
