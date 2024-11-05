use std::io;

fn main() {
    // Prompt for input with error handling
    let experience = get_input("Enter the employee's experience (in years): ");
    let age = get_input("Enter the employee's age: ");

    // Calculate the incentive based on experience and age
    let incentive = calculate_incentive(experience, age);

    // Output the result
    println!("The annual incentive for the employee is: N{}", incentive);
}

// Function to calculate the annual incentive
fn calculate_incentive(experience: u32, age: u32) -> u32 {
    if experience >= 5 { // Assuming experienced is someone with at least 5 years of experience
        if age >= 40 {
            return 1_560_000;
        } else if age >= 30 {
            return 1_480_000;
        } else if age >= 28 {
            return 1_300_000;
        }
    }

    // For inexperienced employees or those who don't match the criteria
    100_000
}

// Function to get a valid u32 input from the user
fn get_input(prompt: &str) -> u32 {
    loop {
        // Print the prompt
        println!("{}", prompt);

        // Initialize a string to store input
        let mut input = String::new();
        
        // Read the input
        io::stdin().read_line(&mut input).expect("Failed to read line");

        // Try to parse the input to u32
        match input.trim().parse::<u32>() {
            Ok(num) if num > 0 => return num, // Return the valid input
            _ => println!("Invalid input! Please enter a positive number."),
        }
    }
}
