use std::io;

fn main() {
    // Prompt for user input
    println!("Enter a number:");

    // Initialize the input string
    let mut input1 = String::new();

    // Read input from user
    io::stdin().read_line(&mut input1).expect("Failed to read input");

    // Parse the input to an integer, handle invalid input gracefully
    let num: i32 = match input1.trim().parse() {
        Ok(n) => n,  // If parsing is successful
        Err(_) => {
            println!("Invalid input. Please enter a valid integer.");
            return; // Exit early if input is invalid
        }
    };

    // Loop until the number reaches 10
    let mut num = num; // Ensure num is mutable after parsing
    while num < 10 {
        println!("Inside loop, number value is {}", num);
        num += 1;
    }

    // After the loop ends
    println!("Outside loop, number value is {}", num);
}
