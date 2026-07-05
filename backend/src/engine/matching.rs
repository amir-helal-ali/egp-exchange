use rust_decimal::Decimal;
use uuid::Uuid;

use super::orderbook::{Order, OrderBook, TradeEvent};

pub struct MatchingEngine {
    pub orderbook: OrderBook,
    pub pair: (String, String),
}

impl MatchingEngine {
    pub fn new(base: String, quote: String) -> Self {
        Self {
            orderbook: OrderBook::new(),
            pair: (base, quote),
        }
    }

    pub fn process_limit(
        &mut self,
        order: &Order,
    ) -> (Vec<TradeEvent>, Option<Order>) {
        let mut trades = Vec::new();
        let mut remaining = order.quantity - order.filled;

        match order.side.as_str() {
            "buy" => {
                while remaining > Decimal::ZERO {
                    let best_ask = self.orderbook.best_ask().map(|(p, q)| (*p, *q));
                    let (ask_price, ask_qty) = match best_ask {
                        Some((p, q)) => (p, q),
                        None => break,
                    };

                    if let Some(limit_price) = order.price {
                        if ask_price > limit_price {
                            break;
                        }
                    }

                    let trade_qty = remaining.min(ask_qty);
                    let total = ask_price * trade_qty;

                    trades.push(TradeEvent {
                        buy_order_id: order.id,
                        sell_order_id: Uuid::default(),
                        buy_user_id: order.user_id,
                        sell_user_id: Uuid::default(),
                        base_currency: self.pair.0.clone(),
                        quote_currency: self.pair.1.clone(),
                        price: ask_price,
                        quantity: trade_qty,
                        total,
                        taker_side: "buy".into(),
                    });

                    remaining -= trade_qty;
                    self.orderbook.remove_ask(&ask_price, trade_qty);
                }

                if remaining > Decimal::ZERO {
                    let placed = Order {
                        id: order.id,
                        user_id: order.user_id,
                        side: "buy".into(),
                        price: order.price,
                        quantity: order.quantity,
                        filled: order.quantity - remaining,
                    };
                    if placed.price.is_some() {
                        self.orderbook.add_bid(placed.price.unwrap(), remaining);
                    }
                    (trades, Some(placed))
                } else {
                    let filled = Order {
                        id: order.id,
                        user_id: order.user_id,
                        side: "buy".into(),
                        price: order.price,
                        quantity: order.quantity,
                        filled: order.quantity,
                    };
                    (trades, Some(filled))
                }
            }
            "sell" => {
                while remaining > Decimal::ZERO {
                    let best_bid = self.orderbook.best_bid().map(|(p, q)| (*p, *q));
                    let (bid_price, bid_qty) = match best_bid {
                        Some((p, q)) => (p, q),
                        None => break,
                    };

                    if let Some(limit_price) = order.price {
                        if bid_price < limit_price {
                            break;
                        }
                    }

                    let trade_qty = remaining.min(bid_qty);
                    let total = bid_price * trade_qty;

                    trades.push(TradeEvent {
                        buy_order_id: Uuid::default(),
                        sell_order_id: order.id,
                        buy_user_id: Uuid::default(),
                        sell_user_id: order.user_id,
                        base_currency: self.pair.0.clone(),
                        quote_currency: self.pair.1.clone(),
                        price: bid_price,
                        quantity: trade_qty,
                        total,
                        taker_side: "sell".into(),
                    });

                    remaining -= trade_qty;
                    self.orderbook.remove_bid(&bid_price, trade_qty);
                }

                if remaining > Decimal::ZERO {
                    let placed = Order {
                        id: order.id,
                        user_id: order.user_id,
                        side: "sell".into(),
                        price: order.price,
                        quantity: order.quantity,
                        filled: order.quantity - remaining,
                    };
                    if placed.price.is_some() {
                        self.orderbook.add_ask(placed.price.unwrap(), remaining);
                    }
                    (trades, Some(placed))
                } else {
                    let filled = Order {
                        id: order.id,
                        user_id: order.user_id,
                        side: "sell".into(),
                        price: order.price,
                        quantity: order.quantity,
                        filled: order.quantity,
                    };
                    (trades, Some(filled))
                }
            }
            _ => (vec![], None),
        }
    }

    pub fn process_market(
        &mut self,
        order: &Order,
    ) -> (Vec<TradeEvent>, Option<Order>) {
        let order_with_price = Order {
            price: None,
            ..order.clone()
        };
        self.process_limit(&order_with_price)
    }

    pub fn cancel_order(&mut self, side: &str, price: &Decimal, quantity: Decimal) {
        match side {
            "buy" => self.orderbook.remove_bid(price, quantity),
            "sell" => self.orderbook.remove_ask(price, quantity),
            _ => {}
        }
    }
}
