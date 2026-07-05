use rust_decimal::Decimal;
use uuid::Uuid;

use crate::db;
use crate::models::FuturesPosition;

pub struct FuturesEngine;

impl FuturesEngine {
    pub fn calculate_liquidation(
        entry_price: Decimal,
        leverage: i32,
        side: &str,
    ) -> Decimal {
        let lev = Decimal::from(leverage);
        match side {
            "long" => entry_price / (Decimal::ONE + Decimal::ONE / lev),
            "short" => entry_price / (Decimal::ONE - Decimal::ONE / lev),
            _ => Decimal::ZERO,
        }
    }

    pub fn calculate_pnl(
        entry_price: Decimal,
        mark_price: Decimal,
        quantity: Decimal,
        side: &str,
    ) -> Decimal {
        match side {
            "long" => (mark_price - entry_price) * quantity,
            "short" => (entry_price - mark_price) * quantity,
            _ => Decimal::ZERO,
        }
    }

    pub fn calculate_margin(
        quantity: Decimal,
        entry_price: Decimal,
        leverage: i32,
    ) -> Decimal {
        quantity * entry_price / Decimal::from(leverage)
    }

    pub async fn check_liquidation(
        pool: &sqlx::PgPool,
        position: &FuturesPosition,
        mark_price: Decimal,
    ) -> Option<FuturesPosition> {
        let liq = Self::calculate_liquidation(position.entry_price, position.leverage, &position.side);

        let triggered = match position.side.as_str() {
            "long" => mark_price <= liq,
            "short" => mark_price >= liq,
            _ => false,
        };

        if triggered {
            let pnl = Self::calculate_pnl(
                position.entry_price,
                mark_price,
                position.quantity,
                &position.side,
            );
            if let Ok(closed) = db::futures::close(pool, position.id, pnl).await {
                return Some(closed);
            }
        }
        None
    }

    pub async fn check_tp_sl(
        pool: &sqlx::PgPool,
        position: &FuturesPosition,
        mark_price: Decimal,
    ) -> Option<FuturesPosition> {
        let tp_triggered = match position.side.as_str() {
            "long" => position.take_profit.is_some_and(|tp| mark_price >= tp),
            "short" => position.take_profit.is_some_and(|tp| mark_price <= tp),
            _ => false,
        };
        let sl_triggered = match position.side.as_str() {
            "long" => position.stop_loss.is_some_and(|sl| mark_price <= sl),
            "short" => position.stop_loss.is_some_and(|sl| mark_price >= sl),
            _ => false,
        };

        if tp_triggered || sl_triggered {
            let pnl = Self::calculate_pnl(
                position.entry_price,
                mark_price,
                position.quantity,
                &position.side,
            );
            if let Ok(closed) = db::futures::close(pool, position.id, pnl).await {
                return Some(closed);
            }
        }
        None
    }

    pub async fn open_position(
        pool: &sqlx::PgPool,
        user_id: Uuid,
        pair: &str,
        side: &str,
        quantity: Decimal,
        leverage: i32,
        entry_price: Decimal,
    ) -> Result<FuturesPosition, sqlx::Error> {
        let margin = Self::calculate_margin(quantity, entry_price, leverage);
        let liquidation = Self::calculate_liquidation(entry_price, leverage, side);

        let position = FuturesPosition {
            id: uuid::Uuid::new_v4(),
            user_id,
            pair: pair.to_string(),
            side: side.to_string(),
            quantity,
            entry_price,
            mark_price: entry_price,
            liquidation_price: liquidation,
            leverage,
            margin,
            unrealized_pnl: Decimal::ZERO,
            realized_pnl: Decimal::ZERO,
            status: "open".to_string(),
            take_profit: None,
            stop_loss: None,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };

        db::futures::create(pool, &position).await
    }
}
