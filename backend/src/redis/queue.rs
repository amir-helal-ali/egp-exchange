use ::redis::aio::ConnectionManager;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

const PENDING_DEPOSITS: &str = "queue:deposits";
const PENDING_WITHDRAWALS: &str = "queue:withdrawals";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueueItem {
    pub tx_id: Uuid,
    pub user_id: Uuid,
    pub user_email: String,
    pub amount: String,
    pub currency: String,
    pub created_at: String,
}

pub async fn push_deposit(
    redis: &mut ConnectionManager,
    item: &QueueItem,
) -> ::redis::RedisResult<()> {
    let json = serde_json::to_string(item).unwrap();
    ::redis::cmd("LPUSH")
        .arg(PENDING_DEPOSITS)
        .arg(&json)
        .query_async::<_, ()>(redis)
        .await?;
    Ok(())
}

pub async fn push_withdrawal(
    redis: &mut ConnectionManager,
    item: &QueueItem,
) -> ::redis::RedisResult<()> {
    let json = serde_json::to_string(item).unwrap();
    ::redis::cmd("LPUSH")
        .arg(PENDING_WITHDRAWALS)
        .arg(&json)
        .query_async::<_, ()>(redis)
        .await?;
    Ok(())
}

pub async fn get_pending_deposits(
    redis: &mut ConnectionManager,
) -> ::redis::RedisResult<Vec<QueueItem>> {
    let items: Vec<String> = ::redis::cmd("LRANGE")
        .arg(PENDING_DEPOSITS)
        .arg(0)
        .arg(-1)
        .query_async(redis)
        .await?;
    let mut result: Vec<QueueItem> = items
        .iter()
        .filter_map(|s| serde_json::from_str(s).ok())
        .collect();
    result.reverse();
    Ok(result)
}

pub async fn get_pending_withdrawals(
    redis: &mut ConnectionManager,
) -> ::redis::RedisResult<Vec<QueueItem>> {
    let items: Vec<String> = ::redis::cmd("LRANGE")
        .arg(PENDING_WITHDRAWALS)
        .arg(0)
        .arg(-1)
        .query_async(redis)
        .await?;
    let mut result: Vec<QueueItem> = items
        .iter()
        .filter_map(|s| serde_json::from_str(s).ok())
        .collect();
    result.reverse();
    Ok(result)
}

pub async fn remove_deposit(
    redis: &mut ConnectionManager,
    tx_id: Uuid,
) -> ::redis::RedisResult<()> {
    remove_item(redis, PENDING_DEPOSITS, tx_id).await
}

pub async fn remove_withdrawal(
    redis: &mut ConnectionManager,
    tx_id: Uuid,
) -> ::redis::RedisResult<()> {
    remove_item(redis, PENDING_WITHDRAWALS, tx_id).await
}

async fn remove_item(
    redis: &mut ConnectionManager,
    key: &str,
    tx_id: Uuid,
) -> ::redis::RedisResult<()> {
    let items: Vec<String> = ::redis::cmd("LRANGE")
        .arg(key)
        .arg(0)
        .arg(-1)
        .query_async(redis)
        .await?;
    for item in &items {
        if let Ok(qi) = serde_json::from_str::<QueueItem>(item) {
            if qi.tx_id == tx_id {
                let _: i32 = ::redis::cmd("LREM")
                    .arg(key)
                    .arg(1)
                    .arg(item)
                    .query_async(redis)
                    .await?;
                break;
            }
        }
    }
    Ok(())
}

pub async fn get_queue_position(
    redis: &mut ConnectionManager,
    tx_id: Uuid,
    tx_type: &str,
) -> ::redis::RedisResult<Option<i64>> {
    let key = match tx_type {
        "deposit" => PENDING_DEPOSITS,
        "withdrawal" => PENDING_WITHDRAWALS,
        _ => return Ok(None),
    };
    let items: Vec<String> = ::redis::cmd("LRANGE")
        .arg(key)
        .arg(0)
        .arg(-1)
        .query_async(redis)
        .await?;
    for (i, item) in items.iter().enumerate() {
        if let Ok(qi) = serde_json::from_str::<QueueItem>(item) {
            if qi.tx_id == tx_id {
                return Ok(Some(i as i64));
            }
        }
    }
    Ok(None)
}
