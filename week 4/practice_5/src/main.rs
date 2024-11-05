use std::io;

fn main() {
    // Prompt the user to enter their height
    println!("Enter your height (in centimeters):");

    // Create a mutable String to store the user input
    let mut input = String::new();

    // Read the user input from stdin
    io::stdin().read_line(&mut input).expect("Failed to read line");

    // Try to parse the input as a floating-point number (height)
    let height: f64 = match input.trim().parse() {
        Ok(num) => num, // Successfully parsed height
        Err(_) => {
            println!("Please enter a valid number for height.");
            return; // Exit early if the input is invalid
        }
    };

    // Output based on the height
    if height > 170.0 {
        println!("You are tall.");
    } else if height < 150.0 {
        println!("You are short.");
    } else {
        println!("You are of average height.");
    }
}
