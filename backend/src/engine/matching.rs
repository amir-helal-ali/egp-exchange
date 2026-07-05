use rust_decimal::Decimal;

use super::orderbook::{LimitOrder, Order, OrderBook, TradeEvent};

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
                    let best_ask = self.orderbook.best_ask().map(|(p, orders)| (*p, orders));
                    let (ask_price, ask_orders) = match best_ask {
                        Some((p, o)) => (p, o),
                        None => break,
                    };

                    if let Some(limit_price) = order.price {
                        if ask_price > limit_price {
                            break;
                        }
                    }

                    let ask_qty: Decimal = ask_orders.iter().map(|o| o.quantity).sum();
                    let trade_qty = remaining.min(ask_qty);
                    let total = ask_price * trade_qty;

                    let removed = self.orderbook.remove_ask(&ask_price, trade_qty);
                    let counterparty = removed.first().cloned().unwrap_or(LimitOrder {
                        id: order.id,
                        user_id: order.user_id,
                        quantity: trade_qty,
                    });

                    trades.push(TradeEvent {
                        buy_order_id: order.id,
                        sell_order_id: counterparty.id,
                        buy_user_id: order.user_id,
                        sell_user_id: counterparty.user_id,
                        base_currency: self.pair.0.clone(),
                        quote_currency: self.pair.1.clone(),
                        price: ask_price,
                        quantity: trade_qty,
                        total,
                        taker_side: "buy".into(),
                    });

                    remaining -= trade_qty;
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
                    if let Some(price) = order.price {
                        self.orderbook.add_bid(price, LimitOrder {
                            id: order.id,
                            user_id: order.user_id,
                            quantity: remaining,
                        });
                    }
                    (trades, Some(placed))
                } else {
                    (trades, Some(Order {
                        id: order.id,
                        user_id: order.user_id,
                        side: "buy".into(),
                        price: order.price,
                        quantity: order.quantity,
                        filled: order.quantity,
                    }))
                }
            }
            "sell" => {
                while remaining > Decimal::ZERO {
                    let best_bid = self.orderbook.best_bid().map(|(p, orders)| (*p, orders));
                    let (bid_price, bid_orders) = match best_bid {
                        Some((p, o)) => (p, o),
                        None => break,
                    };

                    if let Some(limit_price) = order.price {
                        if bid_price < limit_price {
                            break;
                        }
                    }

                    let bid_qty: Decimal = bid_orders.iter().map(|o| o.quantity).sum();
                    let trade_qty = remaining.min(bid_qty);
                    let total = bid_price * trade_qty;

                    let removed = self.orderbook.remove_bid(&bid_price, trade_qty);
                    let counterparty = removed.first().cloned().unwrap_or(LimitOrder {
                        id: order.id,
                        user_id: order.user_id,
                        quantity: trade_qty,
                    });

                    trades.push(TradeEvent {
                        buy_order_id: counterparty.id,
                        sell_order_id: order.id,
                        buy_user_id: counterparty.user_id,
                        sell_user_id: order.user_id,
                        base_currency: self.pair.0.clone(),
                        quote_currency: self.pair.1.clone(),
                        price: bid_price,
                        quantity: trade_qty,
                        total,
                        taker_side: "sell".into(),
                    });

                    remaining -= trade_qty;
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
                    if let Some(price) = order.price {
                        self.orderbook.add_ask(price, LimitOrder {
                            id: order.id,
                            user_id: order.user_id,
                            quantity: remaining,
                        });
                    }
                    (trades, Some(placed))
                } else {
                    (trades, Some(Order {
                        id: order.id,
                        user_id: order.user_id,
                        side: "sell".into(),
                        price: order.price,
                        quantity: order.quantity,
                        filled: order.quantity,
                    }))
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
            "buy" => { self.orderbook.remove_bid(price, quantity); }
            "sell" => { self.orderbook.remove_ask(price, quantity); }
            _ => {}
        }
    }
}
