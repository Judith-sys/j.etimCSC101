use std::io;

fn main() {
    let experience = get_input("Enter the employee's experience (in years): ");
    let age = get_input("Enter the employee's age: ");

    let incentive = calculate_incentive(experience, age);

    println!("The annual incentive for the employee is: N{}", incentive);
}

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

    100_000
}

fn get_input(prompt: &str) -> u32 {
    loop {
        println!("{}", prompt);

        let mut input = String::new();

        io::stdin().read_line(&mut input).expect("Failed to read line");

        match input.trim().parse::<u32>() {
            Ok(num) if num > 0 => return num, // Return the valid input
            _ => println!("Invalid input! Please enter a positive number."),
        }
    }
}
