use std::io;

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();

    // Get base input
    println!("Enter the base of the triangle: ");
    io::stdin()
        .read_line(&mut input1)
        .expect("Failed to read input");
    
    let base: f32 = match input1.trim().parse() {
        Ok(num) if num > 0.0 => num,
        Ok(_) => {
            println!("Base must be a positive number.");
            return;
        }
        Err(_) => {
            println!("Invalid input for base. Please enter a valid number.");
            return;
        }
    };

    // Get height input
    println!("Enter the height of the triangle: ");
    io::stdin()
        .read_line(&mut input2)
        .expect("Failed to read input");

    let height: f32 = match input2.trim().parse() {
        Ok(num) if num > 0.0 => num,
        Ok(_) => {
            println!("Height must be a positive number.");
            return;
        }
        Err(_) => {
            println!("Invalid input for height. Please enter a valid number.");
            return;
        }
    };

    // Calculate and display the area if valid inputs are provided
    let area = (base * height) / 2.0;
    println!("The area of the triangle is: {:.2}", area);
}
