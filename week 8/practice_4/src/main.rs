fn main() {
    // Name vector
    let names = vec!["Mary", "Sam", "Sally", "Greg", "Ade", "Mark", "June", "Ife"];

    // Age vector
    let ages = vec![16, 17, 19, 22, 20, 21, 18, 23];

    println!("\nAge allocation:\n");

    // Loop to iterate elements in the vector
    for i in 0..ages.len() {
        // Printing each name and age
        println!("{} is {} years old", names[i], ages[i]);
    }
}
