#[derive(Debug, Clone)]
pub struct Order {
    pub id: u64,
    pub price: f64,
    pub is_buy: bool,
}
