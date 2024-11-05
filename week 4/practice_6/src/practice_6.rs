use std::io;

fn main() {
    // Prompt for lower bound
    println!("Enter lower bound:");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let lower_bound: i32 = input1.trim().parse().expect("Failed to parse lower bound");

    // Prompt for upper bound
    println!("Enter upper bound:");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let upper_bound: i32 = input2.trim().parse().expect("Failed to parse upper bound");

    // Loop from lower_bound to upper_bound (exclusive)
    for x in lower_bound..upper_bound {
        println!("Count level is {}", x); // Correct syntax for printing
    }
}
