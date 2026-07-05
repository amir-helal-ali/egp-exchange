use std::collections::BTreeMap;
use rust_decimal::Decimal;
use serde::Serialize;

#[derive(Debug, Clone)]
pub struct Order {
    pub id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub side: String,
    pub price: Option<Decimal>,
    pub quantity: Decimal,
    pub filled: Decimal,
}

#[derive(Debug, Clone)]
pub struct TradeEvent {
    pub buy_order_id: uuid::Uuid,
    pub sell_order_id: uuid::Uuid,
    pub buy_user_id: uuid::Uuid,
    pub sell_user_id: uuid::Uuid,
    pub base_currency: String,
    pub quote_currency: String,
    pub price: Decimal,
    pub quantity: Decimal,
    pub total: Decimal,
    pub taker_side: String,
}

#[derive(Debug, Default)]
pub struct OrderBook {
    pub bids: BTreeMap<Decimal, Decimal>,
    pub asks: BTreeMap<Decimal, Decimal>,
}

impl OrderBook {
    pub fn new() -> Self {
        Self {
            bids: BTreeMap::new(),
            asks: BTreeMap::new(),
        }
    }

    pub fn add_bid(&mut self, price: Decimal, quantity: Decimal) {
        *self.bids.entry(price).or_insert(Decimal::ZERO) += quantity;
    }

    pub fn add_ask(&mut self, price: Decimal, quantity: Decimal) {
        *self.asks.entry(price).or_insert(Decimal::ZERO) += quantity;
    }

    pub fn remove_bid(&mut self, price: &Decimal, quantity: Decimal) {
        if let Some(entry) = self.bids.get_mut(price) {
            *entry -= quantity;
            if *entry <= Decimal::ZERO {
                self.bids.remove(price);
            }
        }
    }

    pub fn remove_ask(&mut self, price: &Decimal, quantity: Decimal) {
        if let Some(entry) = self.asks.get_mut(price) {
            *entry -= quantity;
            if *entry <= Decimal::ZERO {
                self.asks.remove(price);
            }
        }
    }

    pub fn best_bid(&self) -> Option<(&Decimal, &Decimal)> {
        self.bids.iter().next_back()
    }

    pub fn best_ask(&self) -> Option<(&Decimal, &Decimal)> {
        self.asks.iter().next()
    }

    pub fn snapshot(&self, depth: usize) -> OrderbookSnapshot {
        let bids: Vec<Level> = self
            .bids
            .iter()
            .rev()
            .take(depth)
            .map(|(p, q)| Level {
                price: *p,
                quantity: *q,
            })
            .collect();

        let asks: Vec<Level> = self
            .asks
            .iter()
            .take(depth)
            .map(|(p, q)| Level {
                price: *p,
                quantity: *q,
            })
            .collect();

        OrderbookSnapshot { bids, asks }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Level {
    pub price: Decimal,
    pub quantity: Decimal,
}

#[derive(Debug, Clone, Serialize)]
pub struct OrderbookSnapshot {
    pub bids: Vec<Level>,
    pub asks: Vec<Level>,
}
