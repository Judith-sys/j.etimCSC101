Question 1 
use std::io::{self, Write};

#[derive(Debug)]
struct Participant {
    name: String,
    email: String,
    department: String,
    state: String,
    cgpa: f32,
    level: u32,
    is_class_representative: bool,
}

impl Participant {
    fn can_vote(&self) -> bool {
        self.is_class_representative && self.level > 100 && self.cgpa > 4.0
    }

    fn print_details(&self) {
        println!("--- Candidate Information ---");
        println!("Full Name: {}", self.name);
        println!("Email: {}", self.email);
        println!("Department: {}", self.department);
        println!("State of Origin: {}", self.state);
    }
}

fn prompt_for_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().expect("Failed to flush stdout");
    let mut response = String::new();
    io::stdin().read_line(&mut response).expect("Failed to read input");
    response.trim().to_string()
}

fn get_valid_float_input(prompt: &str, min: f32, max: f32) -> f32 {
    loop {
        let input = prompt_for_input(prompt);
        match input.parse::<f32>() {
            Ok(value) if value >= min && value <= max => return value,
            _ => println!("Please enter a valid number between {} and {}.", min, max),
        }
    }
}

fn get_valid_integer_input(prompt: &str, min: u32) -> u32 {
    loop {
        let input = prompt_for_input(prompt);
        match input.parse::<u32>() {
            Ok(value) if value >= min => return value,
            _ => println!("Please enter a valid number greater than {}.", min),
        }
    }
}

fn get_class_rep_status() -> bool {
    loop {
        let response = prompt_for_input("Is the candidate a class rep? (yes/no): ");
        if response.to_lowercase() == "yes" {
            return true;
        } else if response.to_lowercase() == "no" {
            return false;
        } else {
            println!("Please answer with 'yes' or 'no'.");
        }
    }
}

fn main() {
    const MAX_CANDIDATES: usize = 50;
    let mut participants: Vec<Participant> = Vec::new();

    for i in 0..MAX_CANDIDATES {
        println!("\n---- Candidate #{} ----", i + 1);

        // Gather input for candidate details
        let name = prompt_for_input("Enter full name: ");
        let email = prompt_for_input("Enter email address: ");
        let department = prompt_for_input("Enter department name: ");
        let state = prompt_for_input("Enter state of origin: ");
        
        let cgpa = get_valid_float_input("Enter CGPA (0.0 - 5.0): ", 0.0, 5.0);
        let level = get_valid_integer_input("Enter academic level (e.g., 200 for 200 level): ", 101);

        let is_class_representative = get_class_rep_status();

        // Create a Participant instance
        let participant = Participant {
            name,
            email,
            department,
            state,
            cgpa,
            level,
            is_class_representative,
        };

        // Determine eligibility and print the result
        if participant.can_vote() {
            participant.print_details();
            println!("\nStatus: Eligible to vote!");
        } else {
            println!("\nStatus: Not eligible to vote.");
        }

        participants.push(participant);

        if i == MAX_CANDIDATES - 1 {
            println!("\nMaximum candidate limit reached.");
            break;
        }
    }
}
Question 2 
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