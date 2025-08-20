
pub(crate) mod generator_orders;
pub(crate) mod models;

use generator_orders::generate_orders;
use models::order::Order;

/// Main function - Entry point of the application
/// Sets up the order matching system and processes incoming orders
#[tokio::main]
async fn main() {
    // Generate a stream of 10 random orders
    let mut order_receiver = generate_orders(100);
    
    // Queues to hold unmatched orders
    let mut buy_orders: Vec<Order> = Vec::new();
    let mut sell_orders: Vec<Order> = Vec::new();
    
    // Process each order as it arrives
    while let Some(order) = order_receiver.recv().await {
        // Display the received order
        let order_type = if order.is_buy { "Buy" } else { "Sell" };
        println!("Received: {} Order {{ id: {}, price: {} }}", order_type, order.id, order.price);

        // Handle order matching based on order type
        match order.is_buy {
            true => handle_buy_order(order, &mut buy_orders, &mut sell_orders),
            false => handle_sell_order(order, &mut buy_orders, &mut sell_orders),
        }
    }

    // Display any remaining unmatched orders
    display_unmatched_orders(&buy_orders, &sell_orders);
}

/// Process a buy order by trying to match it with existing sell orders
/// If no match is found, add it to the buy queue for future matching
fn handle_buy_order(
    buy_order: Order,
    buy_queue: &mut Vec<Order>,
    sell_queue: &mut Vec<Order>,
) {
    // Try to find a matching sell order (sell price <= buy price)
    if let Some(matching_sell_index) = sell_queue
        .iter()
        .position(|sell_order| sell_order.price <= buy_order.price)
    {
        // Match found - remove the sell order from queue and display the match
        let matched_sell_order = sell_queue.remove(matching_sell_index);
        println!(
            "Matched: Buy {{ id: {}, price: {} }} <-> Sell {{ id: {}, price: {} }}",
            buy_order.id, buy_order.price, matched_sell_order.id, matched_sell_order.price
        );
    } else {
        // No match found - add buy order to queue for future matching
        buy_queue.push(buy_order);
    }
}

/// Process a sell order by trying to match it with existing buy orders
/// If no match is found, add it to the sell queue for future matching
fn handle_sell_order(
    sell_order: Order,
    buy_queue: &mut Vec<Order>,
    sell_queue: &mut Vec<Order>,
) {
    // Try to find a matching buy order (buy price >= sell price)
    if let Some(matching_buy_index) = buy_queue
        .iter()
        .position(|buy_order| buy_order.price >= sell_order.price)
    {
        // Match found - remove the buy order from queue and display the match
        let matched_buy_order = buy_queue.remove(matching_buy_index);
        println!(
            "Matched: Buy {{ id: {}, price: {} }} <-> Sell {{ id: {}, price: {} }}",
            matched_buy_order.id, matched_buy_order.price, sell_order.id, sell_order.price
        );
    } else {
        // No match found - add sell order to queue for future matching
        sell_queue.push(sell_order);
    }
}

/// Display any remaining unmatched orders
fn display_unmatched_orders(buy_queue: &[Order], sell_queue: &[Order]) {
    // Only display if there are unmatched orders
    if !buy_queue.is_empty() || !sell_queue.is_empty() {
        println!("\nUnmatched orders remaining:");
        
        // Display unmatched buy orders
        for order in buy_queue {
            println!("  Buy  -> id: {}, price: {}", order.id, order.price);
        }
        
        // Display unmatched sell orders
        for order in sell_queue {
            println!("  Sell -> id: {}, price: {}", order.id, order.price);
        }
    }
}