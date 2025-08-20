
pub(crate) mod generator_orders;
pub(crate) mod models;

use generator_orders::generate_orders;
use models::order::Order;


/// Asynchronously generate a vector of random orders with random delays
#[tokio::main]
async fn main() {
    let orders: Vec<Order> = generate_orders(100).await;

    // Queues to hold unmatched orders
    let mut buy_queue: Vec<Order> = Vec::new();
    let mut sell_queue: Vec<Order> = Vec::new();

    for order in orders {
        let side: &'static str = if order.is_buy { "Buy" } else { "Sell" };
        println!("Received: {} Order {{ id: {}, price: {} }}", side, order.id, order.price);

        if order.is_buy {
            // Try to find a matching sell order with price <= buy price
            if let Some(pos) = sell_queue.iter().position(|s| s.price <= order.price) {
                let sell_order = sell_queue.remove(pos);
                println!(
                    "Matched: Buy {{ id: {}, price: {} }} <-> Sell {{ id: {}, price: {} }}",
                    order.id, order.price, sell_order.id, sell_order.price
                );
            } else {
                buy_queue.push(order);
            }
        } else {
            // Sell order: try to find a matching buy order with price >= sell price
            if let Some(pos) = buy_queue.iter().position(|b| b.price >= order.price) {
                let buy_order = buy_queue.remove(pos);
                println!(
                    "Matched: Buy {{ id: {}, price: {} }} <-> Sell {{ id: {}, price: {} }}",
                    buy_order.id, buy_order.price, order.id, order.price
                );
            } else {
                sell_queue.push(order);
            }
        }
    }

    if !buy_queue.is_empty() || !sell_queue.is_empty() {
        println!("\nUnmatched orders remaining:");
        for o in buy_queue {
            println!("  Buy  -> id: {}, price: {}", o.id, o.price);
        }
        for o in sell_queue {
            println!("  Sell -> id: {}, price: {}", o.id, o.price);
        }
    }
}