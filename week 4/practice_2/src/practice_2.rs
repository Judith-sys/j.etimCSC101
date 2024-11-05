use std::io;

fn get_triangle_side(prompt: &str) -> f32 {
    loop {
        println!("{}", prompt);
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        // Try to parse the input to a floating-point number
        match input.trim().parse::<f32>() {
            Ok(value) if value > 0.0 => return value, // Ensure the side is positive
            Ok(_) => {
                println!("The side length must be a positive number.");
            }
            Err(_) => {
                println!("Please enter a valid number.");
            }
        }
    }
}

fn main() {
    println!("Enter the edges of a triangle to calculate its area.");

    // Get valid input for the sides of the triangle
    let a = get_triangle_side("Enter first edge of the triangle: ");
    let b = get_triangle_side("Enter second edge of the triangle: ");
    let c = get_triangle_side("Enter third edge of the triangle: ");

    // Check if the sides form a valid triangle using the triangle inequality theorem
    if a + b > c && a + c > b && b + c > a {
        // Heron's formula for the area of a triangle
        let s = (a + b + c) / 2.0; // Semi-perimeter
        let area = (s * (s - a) * (s - b) * (s - c)).sqrt();
        println!("The area of the triangle is: {:.2}", area);
    } else {
        println!("The provided sides do not form a valid triangle.");
    }
}

