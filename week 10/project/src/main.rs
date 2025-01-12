
// Define a structure for a Laptop
struct Laptop {
    brand: String,
    price: u32,
    quantity: u32,
}

// Implement a method to calculate the total cost of laptops
impl Laptop {
    fn total_cost(&self) -> u32 {
        self.price * self.quantity
    }
}

fn main() {
    // Create instances for each laptop brand
    let hp = Laptop {
        brand: String::from("HP"),
        price: 650_000,
        quantity: 3,
    };
    let ibm = Laptop {
        brand: String::from("IBM"),
        price: 755_000,
        quantity: 3,
    };
    let toshiba = Laptop {
        brand: String::from("Toshiba"),
        price: 550_000,
        quantity: 3,
    };
    let dell = Laptop {
        brand: String::from("Dell"),
        price: 850_000,
        quantity: 3,
    };

    // Calculate total cost for all purchases
    let total_cost = hp.total_cost() + ibm.total_cost() + toshiba.total_cost() + dell.total_cost();

    // Print the total cost
    println!("The total cost of the laptops is: ₦{}", total_cost);
}
