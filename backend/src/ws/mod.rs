use std::collections::HashMap;
use std::sync::Arc;

use axum::extract::ws::{Message, WebSocket};
use serde_json::Value;
use tokio::sync::{RwLock, broadcast};

pub type ChannelSender = broadcast::Sender<String>;
pub type ChannelMap = Arc<RwLock<HashMap<String, ChannelSender>>>;

#[derive(Clone)]
pub struct PubSub {
    pub channels: ChannelMap,
}

impl PubSub {
    pub fn new() -> Self {
        Self {
            channels: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn subscribe(&self, channel: &str) -> broadcast::Receiver<String> {
        let mut map = self.channels.write().await;
        let tx = map
            .entry(channel.to_string())
            .or_insert_with(|| {
                let (tx, _) = broadcast::channel(256);
                tx
            })
            .clone();
        tx.subscribe()
    }

    pub async fn publish(&self, channel: &str, message: &str) {
        let map = self.channels.read().await;
        if let Some(tx) = map.get(channel) {
            let _ = tx.send(message.to_string());
        }
    }

    pub async fn publish_json(&self, channel: &str, value: &Value) {
        if let Ok(msg) = serde_json::to_string(value) {
            self.publish(channel, &msg).await;
        }
    }
}

pub async fn handle_socket(mut socket: WebSocket, pubsub: PubSub) {
    let mut subs: Vec<(String, broadcast::Receiver<String>)> = Vec::new();

    loop {
        // Drain broadcast receivers (non-blocking)
        for (_, rx) in &mut subs {
            loop {
                match rx.try_recv() {
                    Ok(msg) => {
                        if socket.send(Message::Text(msg.into())).await.is_err() {
                            return;
                        }
                    }
                    Err(broadcast::error::TryRecvError::Empty) => break,
                    Err(_) => break,
                }
            }
        }

        // Read from socket with short timeout
        let timeout = tokio::time::sleep(std::time::Duration::from_millis(100));
        tokio::select! {
            msg = socket.recv() => {
                match msg {
                    Some(Ok(Message::Text(text))) => {
                        if let Ok(cmd) = serde_json::from_str::<Value>(&text) {
                            let cmd_type = cmd["type"].as_str().unwrap_or("");
                            match cmd_type {
                                "subscribe" => {
                                    if let Some(channel) = cmd["channel"].as_str() {
                                        let ch = channel.to_string();
                                        if !subs.iter().any(|(c, _)| c == &ch) {
                                            let rx = pubsub.subscribe(channel).await;
                                            subs.push((ch, rx));
                                            let ack = serde_json::json!({
                                                "type": "subscribed",
                                                "channel": channel
                                            });
                                            let _ = socket.send(Message::Text(ack.to_string())).await;
                                        }
                                    }
                                }
                                "unsubscribe" => {
                                    if let Some(channel) = cmd["channel"].as_str() {
                                        subs.retain(|(c, _)| c != channel);
                                    }
                                }
                                "ping" => {
                                    let _ = socket.send(Message::Text(
                                        serde_json::json!({"type": "pong"}).to_string()
                                    )).await;
                                }
                                _ => {}
                            }
                        }
                    }
                    Some(Ok(Message::Close(_))) | None => return,
                    Some(Err(_)) => return,
                    _ => {}
                }
            }
            _ = timeout => {}
        }
    }
}
