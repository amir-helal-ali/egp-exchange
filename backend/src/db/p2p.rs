use rust_decimal::Decimal;
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::{P2pMessage, P2pOffer, P2pOrder};

pub mod offers {
    use super::*;

    pub async fn create(pool: &PgPool, offer: &P2pOffer) -> Result<P2pOffer, sqlx::Error> {
        sqlx::query_as::<_, P2pOffer>(
            r#"INSERT INTO p2p_offers (user_id, type, currency, fiat_currency, price, min_amount, max_amount, available, payment_method)
               VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9) RETURNING *"#,
        )
        .bind(&offer.user_id)
        .bind(&offer.r#type)
        .bind(&offer.currency)
        .bind(&offer.fiat_currency)
        .bind(&offer.price)
        .bind(&offer.min_amount)
        .bind(&offer.max_amount)
        .bind(&offer.available)
        .bind(&offer.payment_method)
        .fetch_one(pool)
        .await
    }

    pub async fn find_active(
        pool: &PgPool,
        r#type: Option<&str>,
        currency: Option<&str>,
    ) -> Result<Vec<P2pOffer>, sqlx::Error> {
        let mut q = "SELECT * FROM p2p_offers WHERE status = 'active'".to_string();
        if r#type.is_some() {
            q.push_str(" AND type = $1");
        }
        if currency.is_some() {
            q.push_str(" AND currency = $2");
        }
        q.push_str(" ORDER BY price ASC");

        let offers = if r#type.is_some() && currency.is_some() {
            sqlx::query_as::<_, P2pOffer>(&q)
                .bind(r#type.unwrap())
                .bind(currency.unwrap())
                .fetch_all(pool)
                .await?
        } else if r#type.is_some() {
            sqlx::query_as::<_, P2pOffer>(&q)
                .bind(r#type.unwrap())
                .fetch_all(pool)
                .await?
        } else {
            sqlx::query_as::<_, P2pOffer>(&q).fetch_all(pool).await?
        };
        Ok(offers)
    }

    pub async fn find_by_id(pool: &PgPool, id: Uuid) -> Result<Option<P2pOffer>, sqlx::Error> {
        sqlx::query_as::<_, P2pOffer>("SELECT * FROM p2p_offers WHERE id = $1")
            .bind(id)
            .fetch_optional(pool)
            .await
    }

    pub async fn update_available(
        pool: &PgPool,
        id: Uuid,
        available: Decimal,
    ) -> Result<P2pOffer, sqlx::Error> {
        sqlx::query_as::<_, P2pOffer>(
            "UPDATE p2p_offers SET available = $2, updated_at = NOW() WHERE id = $1 RETURNING *",
        )
        .bind(id)
        .bind(available)
        .fetch_one(pool)
        .await
    }

    pub async fn deactivate(pool: &PgPool, id: Uuid) -> Result<P2pOffer, sqlx::Error> {
        sqlx::query_as::<_, P2pOffer>(
            "UPDATE p2p_offers SET status = 'inactive', updated_at = NOW() WHERE id = $1 RETURNING *",
        )
        .bind(id)
        .fetch_one(pool)
        .await
    }

    #[allow(dead_code)]
    pub async fn find_by_user(pool: &PgPool, user_id: Uuid) -> Result<Vec<P2pOffer>, sqlx::Error> {
        sqlx::query_as::<_, P2pOffer>(
            "SELECT * FROM p2p_offers WHERE user_id = $1 ORDER BY created_at DESC",
        )
        .bind(user_id)
        .fetch_all(pool)
        .await
    }

    #[allow(dead_code)]
    pub async fn list_all(pool: &PgPool) -> Result<Vec<P2pOffer>, sqlx::Error> {
        sqlx::query_as::<_, P2pOffer>("SELECT * FROM p2p_offers ORDER BY created_at DESC")
            .fetch_all(pool)
            .await
    }
}

pub mod orders {
    use super::*;

    pub async fn create(pool: &PgPool, order: &P2pOrder) -> Result<P2pOrder, sqlx::Error> {
        sqlx::query_as::<_, P2pOrder>(
            r#"INSERT INTO p2p_orders (offer_id, buyer_id, seller_id, amount, total)
               VALUES ($1, $2, $3, $4, $5) RETURNING *"#,
        )
        .bind(&order.offer_id)
        .bind(&order.buyer_id)
        .bind(&order.seller_id)
        .bind(&order.amount)
        .bind(&order.total)
        .fetch_one(pool)
        .await
    }

    pub async fn find_by_id(pool: &PgPool, id: Uuid) -> Result<Option<P2pOrder>, sqlx::Error> {
        sqlx::query_as::<_, P2pOrder>("SELECT * FROM p2p_orders WHERE id = $1")
            .bind(id)
            .fetch_optional(pool)
            .await
    }

    pub async fn update_status(
        pool: &PgPool,
        id: Uuid,
        status: &str,
    ) -> Result<P2pOrder, sqlx::Error> {
        sqlx::query_as::<_, P2pOrder>(
            "UPDATE p2p_orders SET status = $2, updated_at = NOW() WHERE id = $1 RETURNING *",
        )
        .bind(id)
        .bind(status)
        .fetch_one(pool)
        .await
    }

    pub async fn find_by_user(
        pool: &PgPool,
        user_id: Uuid,
    ) -> Result<Vec<P2pOrder>, sqlx::Error> {
        sqlx::query_as::<_, P2pOrder>(
            "SELECT * FROM p2p_orders WHERE buyer_id = $1 OR seller_id = $1 ORDER BY created_at DESC",
        )
        .bind(user_id)
        .fetch_all(pool)
        .await
    }

    pub async fn list_all(pool: &PgPool) -> Result<Vec<P2pOrder>, sqlx::Error> {
        sqlx::query_as::<_, P2pOrder>("SELECT * FROM p2p_orders ORDER BY created_at DESC")
            .fetch_all(pool)
            .await
    }
}

pub mod messages {
    use super::*;

    pub async fn send(pool: &PgPool, order_id: Uuid, sender_id: Uuid, message: &str) -> Result<P2pMessage, sqlx::Error> {
        sqlx::query_as::<_, P2pMessage>(
            "INSERT INTO p2p_messages (order_id, sender_id, message) VALUES ($1, $2, $3) RETURNING *",
        )
        .bind(order_id)
        .bind(sender_id)
        .bind(message)
        .fetch_one(pool)
        .await
    }

    pub async fn by_order(pool: &PgPool, order_id: Uuid) -> Result<Vec<P2pMessage>, sqlx::Error> {
        sqlx::query_as::<_, P2pMessage>(
            "SELECT * FROM p2p_messages WHERE order_id = $1 ORDER BY created_at ASC",
        )
        .bind(order_id)
        .fetch_all(pool)
        .await
    }
}
