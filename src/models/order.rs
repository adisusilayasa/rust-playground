/// Represents a buy or sell order in the system
#[derive(Debug, Clone)]
pub struct Order {
    /// Unique identifier for the order
    pub id: u64,
    /// Price at which the user wants to buy or sell
    pub price: f64,
    /// True if this is a buy order, false if it's a sell order
    pub is_buy: bool,
}
