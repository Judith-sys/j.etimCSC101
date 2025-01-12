fn main() {
    let v = vec![101, 250, 330, 400];
    // vector v owns the object in heap 

    // only a single variable owns the heap memory at any given time 

    let v2 = v;

    // This will print the vector using the debug format specifier.
    println!("{:?}", v2);
}
