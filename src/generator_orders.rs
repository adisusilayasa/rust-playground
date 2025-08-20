use crate::models::order::Order;
use rand::{prelude::ThreadRng, rng, Rng};
use tokio::time::{sleep, Duration};

pub async fn generate_orders(count: usize) -> Vec<Order> {
    let mut orders: Vec<Order> = Vec::<Order>::with_capacity(count);
    let mut rng: ThreadRng = rng();

    for id in 1..=count as u64 {    
        // random delay between 100..500 ms
        let delay_ms: u64 = rng.random_range(100..=500);
        sleep(Duration::from_millis(delay_ms)).await;

        let price: f64 = rng.random_range(80.0..=120.0);
        let is_buy: bool = rng.random_range(0..=1) == 1;

        orders.push(Order { id, price, is_buy });
    }

    orders
}
