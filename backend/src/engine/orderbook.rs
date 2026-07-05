use std::collections::BTreeMap;
use rust_decimal::Decimal;
use serde::Serialize;
use uuid::Uuid;

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

#[derive(Debug, Clone)]
pub struct LimitOrder {
    pub id: Uuid,
    pub user_id: Uuid,
    pub quantity: Decimal,
}

#[derive(Debug, Default)]
pub struct OrderBook {
    pub bids: BTreeMap<Decimal, Vec<LimitOrder>>,
    pub asks: BTreeMap<Decimal, Vec<LimitOrder>>,
}

impl OrderBook {
    pub fn new() -> Self {
        Self {
            bids: BTreeMap::new(),
            asks: BTreeMap::new(),
        }
    }

    pub fn add_bid(&mut self, price: Decimal, order: LimitOrder) {
        self.bids.entry(price).or_default().push(order);
    }

    pub fn add_ask(&mut self, price: Decimal, order: LimitOrder) {
        self.asks.entry(price).or_default().push(order);
    }

    pub fn remove_bid(&mut self, price: &Decimal, quantity: Decimal) -> Vec<LimitOrder> {
        if let Some(orders) = self.bids.get_mut(price) {
            let mut remaining = quantity;
            let mut taken = Vec::new();
            while remaining > Decimal::ZERO && !orders.is_empty() {
                let mut front = orders.remove(0);
                let take = front.quantity.min(remaining);
                remaining -= take;
                if front.quantity > take {
                    orders.insert(0, LimitOrder { quantity: front.quantity - take, ..front });
                    front.quantity = take;
                }
                taken.push(front);
            }
            if orders.is_empty() {
                self.bids.remove(price);
            }
            taken
        } else {
            vec![]
        }
    }

    pub fn remove_ask(&mut self, price: &Decimal, quantity: Decimal) -> Vec<LimitOrder> {
        if let Some(orders) = self.asks.get_mut(price) {
            let mut remaining = quantity;
            let mut taken = Vec::new();
            while remaining > Decimal::ZERO && !orders.is_empty() {
                let mut front = orders.remove(0);
                let take = front.quantity.min(remaining);
                remaining -= take;
                if front.quantity > take {
                    orders.insert(0, LimitOrder { quantity: front.quantity - take, ..front });
                    front.quantity = take;
                }
                taken.push(front);
            }
            if orders.is_empty() {
                self.asks.remove(price);
            }
            taken
        } else {
            vec![]
        }
    }

    pub fn best_bid(&self) -> Option<(&Decimal, &Vec<LimitOrder>)> {
        self.bids.iter().next_back()
    }

    pub fn best_ask(&self) -> Option<(&Decimal, &Vec<LimitOrder>)> {
        self.asks.iter().next()
    }

    pub fn snapshot(&self, depth: usize) -> OrderbookSnapshot {
        let bids: Vec<Level> = self
            .bids
            .iter()
            .rev()
            .take(depth)
            .map(|(p, orders)| Level {
                price: *p,
                quantity: orders.iter().map(|o| o.quantity).sum(),
            })
            .collect();

        let asks: Vec<Level> = self
            .asks
            .iter()
            .take(depth)
            .map(|(p, orders)| Level {
                price: *p,
                quantity: orders.iter().map(|o| o.quantity).sum(),
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
