fn main() {
    let mut count = 0; // Initialize count to 0

    for num in 1..21 {
        if num > 10 {
            println!("(:?) {}", num); // Print numbers greater than 10
            continue; // Skip to the next iteration
        }

        count += 1; // Increment count for values less than or equal to 10
    }

    println!("The count of values greater than 10 (between 1 and 20) is: {}", count);
}
