use std::sync::Arc;
use std::time::{Duration, Instant};

use tokio::sync::{broadcast, RwLock};
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message;
use futures_util::StreamExt;
use rust_decimal::Decimal;
use serde_json::Value;

use crate::models::OrderbookLevel;

#[derive(Debug, Clone)]
pub struct PriceFeed {
    pub symbol: String,
    pub bid: Option<Decimal>,
    pub ask: Option<Decimal>,
    pub last: Option<Decimal>,
    pub bids: Vec<OrderbookLevel>,
    pub asks: Vec<OrderbookLevel>,
    pub circuit_open: bool,
}

impl PriceFeed {
    fn empty(symbol: String) -> Self {
        Self {
            symbol,
            bid: None,
            ask: None,
            last: None,
            bids: Vec::new(),
            asks: Vec::new(),
            circuit_open: false,
        }
    }
}

pub type PriceFeedState = Arc<RwLock<PriceFeed>>;

#[derive(Clone)]
pub struct BinanceConfig {
    pub ws_base: String,
    pub symbols: Vec<String>,
    pub circuit_breaker_timeout: Duration,
}

impl Default for BinanceConfig {
    fn default() -> Self {
        Self {
            ws_base: "wss://stream.binance.com:9443/ws".into(),
            symbols: vec!["btcusdt".into()],
            circuit_breaker_timeout: Duration::from_secs(30),
        }
    }
}

pub async fn start_price_feed(
    config: BinanceConfig,
) -> (PriceFeedState, broadcast::Receiver<PriceFeed>) {
    let state: PriceFeedState = Arc::new(RwLock::new(PriceFeed::empty(config.symbols[0].clone())));
    let (tx, rx) = broadcast::channel(256);

    let state_clone = state.clone();
    let tx_clone = tx.clone();

    tokio::spawn(async move {
        // Use raw WebSocket for individual streams, then subscribe separately
        let url = format!("{}/{}@ticker", config.ws_base, config.symbols[0]);
        let mut last_message = Instant::now();

        loop {
            match connect_async(&url).await {
                Ok((ws_stream, _)) => {
                    tracing::info!("Connected to Binance WebSocket: {}", url);
                    let (_, mut read) = ws_stream.split();
                    last_message = Instant::now();

                    {
                        let mut feed = state_clone.write().await;
                        feed.circuit_open = false;
                    }

                    while let Some(msg) = read.next().await {
                        match msg {
                            Ok(Message::Text(text)) => {
                                last_message = Instant::now();
                                tracing::debug!("Binance WS received: {} bytes", text.len());
                                if let Ok(value) = serde_json::from_str::<Value>(&text) {
                                    process_message(&state_clone, &tx_clone, &value).await;
                                }
                            }
                            Ok(Message::Ping(_)) => {}
                            Ok(Message::Pong(_)) => {}
                            Ok(Message::Close(frame)) => {
                                tracing::warn!("Binance WS closed: {:?}", frame);
                                break;
                            }
                            Err(e) => {
                                tracing::warn!("Binance WS error: {e}");
                                break;
                            }
                            _ => {
                                tracing::debug!("Binance WS unknown message type");
                            }
                        }
                    }
                }
                Err(e) => {
                    tracing::error!("Failed to connect to Binance: {e}");
                }
            }

            if last_message.elapsed() > config.circuit_breaker_timeout {
                tracing::warn!("Binance feed stalled - opening circuit breaker");
                {
                    let mut feed = state_clone.write().await;
                    feed.circuit_open = true;
                }
                let _ = tx_clone.send(PriceFeed {
                    circuit_open: true,
                    ..PriceFeed::empty(config.symbols[0].clone())
                });
            }

            tracing::info!("Reconnecting to Binance in 5 seconds...");
            tokio::time::sleep(Duration::from_secs(5)).await;
        }
    });

    (state, rx)
}

async fn process_message(state: &PriceFeedState, tx: &broadcast::Sender<PriceFeed>, value: &Value) {
    if let Some(event_type) = value.get("e").and_then(|v| v.as_str()) {
        if let Some(last) = value.get("c").and_then(|v| v.as_str()) {
            let last_dec = Decimal::from_str_exact(last).ok();
            tracing::debug!("Ticker: e={}, c={}, parsed={:?}", event_type, last, last_dec);
            let mut feed = state.write().await;
            feed.last = last_dec;
            feed.bid = value
                .get("b")
                .and_then(|v| v.as_str())
                .and_then(|s| Decimal::from_str_exact(s).ok());
            feed.ask = value
                .get("a")
                .and_then(|v| v.as_str())
                .and_then(|s| Decimal::from_str_exact(s).ok());
            let snapshot = feed.clone();
            let _ = tx.send(snapshot);
            return;
        } else {
            tracing::debug!("Ticker event {} but no 'c' field", event_type);
        }
    }

    if let Some(_last_update) = value.get("lastUpdateId") {
        if let Some(bids) = value.get("bids").and_then(|v| v.as_array()) {
            if let Some(asks) = value.get("asks").and_then(|v| v.as_array()) {
                let mut feed = state.write().await;
                feed.bids = bids
                    .iter()
                    .filter_map(|level| {
                        let price = level[0].as_str()?;
                        let qty = level[1].as_str()?;
                        Some(OrderbookLevel {
                            price: Decimal::from_str_exact(price).ok()?,
                            quantity: Decimal::from_str_exact(qty).ok()?,
                        })
                    })
                    .collect();
                feed.asks = asks
                    .iter()
                    .filter_map(|level| {
                        let price = level[0].as_str()?;
                        let qty = level[1].as_str()?;
                        Some(OrderbookLevel {
                            price: Decimal::from_str_exact(price).ok()?,
                            quantity: Decimal::from_str_exact(qty).ok()?,
                        })
                    })
                    .collect();
            }
        }
    }
}
