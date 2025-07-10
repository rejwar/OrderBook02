use crate::order::{Order , OrderType};
#[derive(Debug)]

pub struct OrderBook {
    pub Bids: Vec<Order>,
    pub Asks: Vec<Order>,
}

impl OrderBook {
    pub fn New() -> Self {
        Self {
            Bids: Vec::new(),
            Asks: Vec::new(),
        }
    }

    pub fn AddOrder(&mut self , order: Order) {
        match order.OrderType {
            OrderType::Buy => self.Bids.push(order),
            OrderType::Sell => self.Asks.push(order),
        }
    }
}