use std::io;

fn main() {
    // Declare mutable variables for user input
    let mut input1 = String::new(); // For user's name
    let mut input2 = String::new(); // For user's age

    // Get the user's name
    print!("Enter your name: ");
    io::stdin().read_line(&mut input1).expect("Failed to read line");
    input1 = input1.trim().to_string(); // Remove newline and trim spaces

    // Get the user's age
    print!("Enter your age: ");
    io::stdin().read_line(&mut input2).expect("Failed to read line");

    // Try to parse the age as a u32 (unsigned integer)
    let age: u32 = match input2.trim().parse() {
        Ok(num) => num, // Successfully parsed age
        Err(_) => {
            println!("Invalid input for age. Please enter a valid number.");
            return; // Exit the program early if the input is invalid
        }
    };

    // Output the user's name and age
    println!("Hello, {}! You are {} years old.", input1, age);
}
