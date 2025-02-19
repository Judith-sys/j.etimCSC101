use std::io;

fn main() {
    // Create an empty vector "city"
    let mut city: Vec<String> = Vec::new();

    // Print the city vector's length
    println!("The City vector has {} elements.", city.len());

    // Prompt the user for the number of cities to enter
    let mut input1 = String::new();
    println!("How many cities do you want to enter?");

    io::stdin().read_line(&mut input1).expect("Failed to read input");

    let city_num:i32 = input1.trim() .parse().expect("Invalid input. Please enter a number.");

    for count in 0..city_num {
        let mut input2 = String::new();

        println!("Enter City {}:", count + 1);

        std::io::stdin().read_line(&mut input2) .expect("Failed to read input");

        let new_city = input2.trim().parse().expect("Invalid input");

        city.push(new_city);
    }

    print!("Your preferred cities are:\n'");
    
    let mut count=1;
    //loop to iterate elements in vector
    for i in city
    {
      //interating through i on the vector
      println!("{} {}",count, i);
      count+=1;
    }
}
