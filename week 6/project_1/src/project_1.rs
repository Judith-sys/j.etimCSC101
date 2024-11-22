use std::io;

fn main() {
    // Menu items and prices
    let menu = [
        ("Poundo Yam/Edinkaiko Soup", 3200),
        ("Fried Rice & Chicken", 3000),
        ("Amala & Ewedu Soup", 2500),
        ("Eba & Egusi Soup", 2000),
        ("White Rice & Stew", 2500),
    ];

    let mut total_cost = 0;

    loop {
        // Food menu
        println!("Food Menu:");
        for (index, item) in menu.iter().enumerate() {
            println!("{}: {} (N{})", (index + 1), item.0, item.1);
        }

        println!("\nEnter the number of the food item you want to order (or '0' to finish): ");
        
        // Get the food choice from the user
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");
        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a number.");
                continue;
            }
        };

        if choice == 0 {
            break; // Exit loop if '0' is entered
        }

        if choice < 1 || choice > 5 {
            println!("Invalid choice. Please select a valid food item.");
            continue;
        }

        // Get the quantity from the user
        println!("Enter the quantity: ");
        let mut quantity = String::new();
        io::stdin().read_line(&mut quantity).expect("Failed to read line");
        let quantity: u32 = match quantity.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a valid quantity.");
                continue;
            }
        };

        // Calculate the cost for the selected item
        let item_price = menu[(choice - 1) as usize].1;
        let item_name = menu[(choice - 1) as usize].0;
        let cost = item_price * quantity;

        // Add to total cost
        total_cost += cost;
        
        println!("You ordered: {} x {} = N{}", quantity, item_name, cost);
    }

    // Check if the total cost exceeds N10,000 and apply discount if necessary
    if total_cost > 10000 {
        let discount = total_cost as f32 * 0.05; // 5% discount
        let final_cost = total_cost as f32 - discount;
        println!("\nTotal cost before discount: N{}", total_cost);
        println!("Discount applied: N{}", discount);
        println!("Final total cost after discount: N{}", final_cost);
    } else {
        println!("\nTotal cost: N{}", total_cost);
    }
}
