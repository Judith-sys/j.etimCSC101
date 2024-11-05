fn main() {
    let mut x = 0; // Initialize x to 0
    while x < 15 { // Loop while x is less than 15
        x += 1; // Increment x by 1
        println!("x={}", x); // Print the current value of x

        // No need for an explicit "if" for breaking; the loop will end when x reaches 15
    }
}