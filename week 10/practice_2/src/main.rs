fn main() {
    let v = vec![10, 20, 30];
    // vector v owns the object in heap

    let v2 = v.clone();  // Clone v to create a copy

    display(v2);  

    // v is still usable here because it was not moved
    println!("In main {:?}", v);
}

fn display(v: Vec<i32>) {
    println!("inside display {:?}", v);
}
