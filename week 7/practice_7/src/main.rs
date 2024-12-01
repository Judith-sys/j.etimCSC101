fn main() {
    // Array with explicit integer datatype
    let arr1: [i32; 4] = [10, 20, 30, 40];
    println!("\nArray with data type");
    println!("array is {:?}", arr1);
    println!("array size is: {}", arr1.len());

    // Array with implicit float datatype (using f32)
    let arr2 = [10.0, 4.0, 20.0, 7.0, 30.4, 40.9, 51.2, 72.2];  // All values should be f32
    println!("\nArray with implicit data type");
    println!("array is {:?}", arr2);
    println!("array size is :{}", arr2.len());

    // Array with default values that creates and initializes all its elements with -1.
    let arr3: [i32; 8] = [-1; 8];
    println!("\nArray with default values");
    println!("array is {:?}", arr3);
    println!("array size is :{}", arr3.len());
}
