#[derive(Debug, Clone)]

pub enum OrderType {
    Buy,
    Sell,
}

#[derive(Debug , Clone)]

pub struct Order {
    pub Id: u32 ,
    pub OrderType: OrderType,
    pub Price: f64,
    pub Quantity: f64,
}

impl Order {
    pub fn New(Id: u32 , OrderType: OrderType , Price: f64 , Quantity: f64 ) -> Self {
        Self {Id, OrderType , Price , Quantity}
    }
}