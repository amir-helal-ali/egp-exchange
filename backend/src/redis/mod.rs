pub mod queue;

use ::redis::aio::ConnectionManager;

pub async fn connect(redis_url: &str) -> Result<ConnectionManager, ::redis::RedisError> {
    let client = ::redis::Client::open(redis_url)?;
    let mgr = ConnectionManager::new(client).await?;
    tracing::info!("Connected to Redis");
    Ok(mgr)
}
