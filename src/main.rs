mod order;
mod orderbook;

use order::{Order, OrderType};
use orderbook::OrderBook;

fn main() {
    let mut MyOrderBook = orderbook::New();

    let BuyOrder = Order::New(1, OrderType::Buy, 100.9, 10.0);
    MyOrderBook.AddOrder(BuyOrder);

    let SellOrder = order::New(2 , OrderType::Sell , 101.00 , 5.0);
    MyOrderBook.AddOrder(SellOrder);

    println!("{:?}", MyOrderBook);
}