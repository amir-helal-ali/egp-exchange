pub mod migrations;
pub mod orders;
pub mod trades;
pub mod users;
pub mod wallets;
pub mod manual_transactions;
pub mod futures;
pub mod p2p;
pub mod settings;

use sqlx::PgPool;

pub async fn connect(database_url: &str) -> Result<PgPool, sqlx::Error> {
    let pool = PgPool::connect(database_url).await?;
    tracing::info!("Connected to PostgreSQL");
    Ok(pool)
}
