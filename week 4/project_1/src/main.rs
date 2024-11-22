use std::io;

fn main() {
   
    let mut input = String::new();
    println!("Enter the coefficient a:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let a: f64 = input.trim().parse().expect("Please enter a valid number for coefficient a");


    input.clear();
    println!("Enter the coefficient b:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let b: f64 = input.trim().parse().expect("Please enter a valid number for coefficient b");

   
    input.clear();
    println!("Enter the coefficient c:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let c: f64 = input.trim().parse().expect("Please enter a valid number for coefficient c");

    
    let discriminant = b * b - 4.0 * a * c;

   
    if discriminant > 0.0 {
        
        let root1 = (-b + discriminant.sqrt()) / (2.0 * a);
        let root2 = (-b - discriminant.sqrt()) / (2.0 * a);
        println!("The equation has two real roots: x1 = {} and x2 = {}", root1, root2);
    } else if discriminant == 0.0 {
        // One real root
        let root = -b / (2.0 * a);
        println!("The equation has one real root: x = {}", root);
    } else {
        // No real roots (complex roots)
        println!("The equation has no real roots. The roots are complex.");
    }
}
