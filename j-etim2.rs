use std::io::{self, Write};

fn determine_incentive(published_papers: u32) -> u32 {
    match published_papers {
        0..=2 => 100_000,          // Less than 3 papers
        3..=5 => 500_000,          // 3 to 5 papers
        6..=9 => 800_000,          // More than 5 but less than 10 papers
        _ => 1_000_000,            // 10 or more papers
    }
}

fn collect_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap(); // Ensure prompt is printed immediately
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string()  // Remove any extra whitespace and return
}

fn get_valid_number_input(prompt: &str) -> u32 {
    loop {
        let user_input = collect_input(prompt);
        match user_input.parse::<u32>() {
            Ok(number) if number >= 1 => return number,
            _ => println!("Invalid input. Please enter a valid number."),
        }
    }
}

fn main() {
    const MAX_EMPLOYEES: usize = 100;  // Maximum number of staff members
    let mut employee_data = Vec::new();

    for idx in 0..MAX_EMPLOYEES {
        println!("\nProcessing staff member #{}", idx + 1);

        // Get staff name
        let staff_name = collect_input("Enter the name of the staff member: ");
        
        // Get the number of papers published
        let papers_count = get_valid_number_input("Enter the number of papers published: ");

        // Calculate the incentive based on the number of papers published
        let incentive_amount = determine_incentive(papers_count);

        // Store the staff data
        employee_data.push((staff_name.clone(), incentive_amount));

        // Print the incentive immediately after processing the current staff member
        if let Some(last_entry) = employee_data.last() {
            println!("Staff: {} | Incentive: N{}", last_entry.0, last_entry.1);
        }

        // Stop after 100 staff members
        if idx == MAX_EMPLOYEES - 1 {
            println!("\nThe maximum limit of {} staff members has been reached.", MAX_EMPLOYEES);
            break;
        }
    }

    // Optionally, display all staff incentives at the end
    println!("\n--- Staff Incentives Summary ---");
    for (name, incentive) in &employee_data {
        println!("Staff: {} | Incentive: N{}", name, incentive);
    }
}