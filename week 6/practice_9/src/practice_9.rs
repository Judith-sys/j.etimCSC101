fn main() {
    // Assign values to A and B
    let A = 10;
    let B = 20;

    // Print values of A and B
    println!("Value of A: {}", A);
    println!("Value of B: {}", B);

    // Compare A and B
    let mut res = A > B;
    println!("A greater than B: {}", res);

    res = A < B;
    println!("A lesser than B: {}", res);

    res = A >= B;
    println!("A greater than or equal to B: {}", res);

    res = A <= B;
    println!("A lesser than or equal to B: {}", res);

    res = A == B;
    println!("A is equal to B: {}", res);

    res = A != B;
    println!("A is not equal to B: {}", res);
}
