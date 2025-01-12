fn main() {
    let v = vec![20, 40, 60, 80];
    // vector v owns the object in heap

    let v2 = v;
    let v2_return = display(v2); // Ownership of v2 is moved to display and returned back

    // Now v2_return owns the vector
    println!("In main {:?}", v2_return); // We print the returned vector, not v
}

fn display(v: Vec<i32>) -> Vec<i32> {
    // v is moved into the function, then returned
    println!("inside display {:?}", v);
    return v;  // Return the vector back
}
