use crate::models::order::Order;
use rand::Rng;
use tokio::sync::mpsc::{self, Receiver};
use tokio::time::{sleep, Duration};

/// Generate a stream of random buy/sell orders with random delays
/// 
/// # Arguments
/// * `count` - The number of orders to generate
/// 
/// # Returns
/// A receiver that will receive the generated orders asynchronously
pub fn generate_orders(count: usize) -> Receiver<Order> {
    // Create a channel with enough buffer space to prevent blocking
    let (sender, receiver) = mpsc::channel::<Order>(count);

    // Spawn an asynchronous task to generate orders
    tokio::spawn(async move {
        // Generate the specified number of orders
        for id in 1..=count as u64 {
            // Add a random delay between orders (100-500 ms)
            let delay_ms: u64 = rand::thread_rng().gen_range(100..=500);
            sleep(Duration::from_millis(delay_ms)).await;

            // Generate random order properties
            let price: f64 = rand::thread_rng().gen_range(80.0..=120.0);
            let is_buy: bool = rand::thread_rng().gen_range(0..=1) == 1;

            // Create and send the order
            // Ignore send errors if the receiver has been dropped
            let _ = sender.send(Order { id, price, is_buy }).await;
        }
    });

    receiver
}
