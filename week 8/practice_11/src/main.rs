fn main() {
    // an array of numbers
    let numbers = [1, 2, 3, 4, 5]; 
    println!("Original array =  {:?}", numbers);

    // create slice of 2nd and 3rd elements
    let slice1 = &numbers[1..3];
    println!("2nd and 3rd elements sliced =  {:?}", slice1);

    // omit the start index
    let slice2 = &numbers[..3];
    println!("Index 0 to index 3 sliced = {:?}", slice2);

    // omit the end index
    let slice3 = &numbers[2..];
    println!("Index 2 to end sliced = {:?}", slice3);

    // omit the start index and the end index (reference the whole array)
    let slice4 = &numbers[..];
    println!("index 0 to index 5 sliced = {:?}", slice4);
}
