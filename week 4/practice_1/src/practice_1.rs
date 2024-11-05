use std::io;

fn main() {
    println!("\nStudent Information Management System!");

    // 1. Input name
    println!("\nPlease enter your name.");
    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read input");

    // Trim any trailing newlines from input
    let name = name.trim();
    println!("Your name is: {}", name);

    // Input age
    println!("\nEnter your age.");
    let mut age = String::new();
    io::stdin()
        .read_line(&mut age)
        .expect("Failed to read input");

    // Try to parse age to an integer, with error handling
    let age: u32 = match age.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input for age. Please enter a valid number.");
            return; // Exit early if the input is invalid
        }
    };

    println!("Your age is: {}", age);
}
