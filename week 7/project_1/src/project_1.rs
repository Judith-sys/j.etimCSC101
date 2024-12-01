use std::io;

fn area_of_trapezium(base1: f64, base2: f64, height: f64) -> f64 {
    (height / 2.0) * (base1 + base2)
}

fn area_of_rhombus(diagonal1: f64, diagonal2: f64) -> f64 {
    0.5 * diagonal1 * diagonal2
}

fn area_of_parallelogram(base: f64, altitude: f64) -> f64 {
    base * altitude
}

fn area_of_cube(side: f64) -> f64 {
    6.0 * side * side
}

fn volume_of_cylinder(radius: f64, height: f64) -> f64 {
    std::f64::consts::PI * radius * radius * height
}

fn main() {
    loop {
        println!("Select the shape you want to calculate the area or volume for:");
        println!("1. Trapezium");
        println!("2. Rhombus");
        println!("3. Parallelogram");
        println!("4. Cube");
        println!("5. Cylinder");
        println!("6. Exit");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");
        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input, please enter a valid number.");
                continue;
            }
        };

        match choice {
            1 => {
                let mut base1 = String::new();
                let mut base2 = String::new();
                let mut height = String::new();

                println!("Enter the first base of the trapezium:");
                io::stdin().read_line(&mut base1).expect("Failed to read line");
                let base1: f64 = base1.trim().parse().expect("Invalid input");

                println!("Enter the second base of the trapezium:");
                io::stdin().read_line(&mut base2).expect("Failed to read line");
                let base2: f64 = base2.trim().parse().expect("Invalid input");

                println!("Enter the height of the trapezium:");
                io::stdin().read_line(&mut height).expect("Failed to read line");
                let height: f64 = height.trim().parse().expect("Invalid input");

                let area = area_of_trapezium(base1, base2, height);
                println!("The area of the trapezium is: {}", area);
            },
            2 => {
                let mut diagonal1 = String::new();
                let mut diagonal2 = String::new();

                println!("Enter the first diagonal of the rhombus:");
                io::stdin().read_line(&mut diagonal1).expect("Failed to read line");
                let diagonal1: f64 = diagonal1.trim().parse().expect("Invalid input");

                println!("Enter the second diagonal of the rhombus:");
                io::stdin().read_line(&mut diagonal2).expect("Failed to read line");
                let diagonal2: f64 = diagonal2.trim().parse().expect("Invalid input");

                let area = area_of_rhombus(diagonal1, diagonal2);
                println!("The area of the rhombus is: {}", area);
            },
            3 => {
                let mut base = String::new();
                let mut altitude = String::new();

                println!("Enter the base of the parallelogram:");
                io::stdin().read_line(&mut base).expect("Failed to read line");
                let base: f64 = base.trim().parse().expect("Invalid input");

                println!("Enter the altitude of the parallelogram:");
                io::stdin().read_line(&mut altitude).expect("Failed to read line");
                let altitude: f64 = altitude.trim().parse().expect("Invalid input");

                let area = area_of_parallelogram(base, altitude);
                println!("The area of the parallelogram is: {}", area);
            },
            4 => {
                let mut side = String::new();

                println!("Enter the length of the side of the cube:");
                io::stdin().read_line(&mut side).expect("Failed to read line");
                let side: f64 = side.trim().parse().expect("Invalid input");

                let area = area_of_cube(side);
                println!("The surface area of the cube is: {}", area);
            },
            5 => {
                let mut radius = String::new();
                let mut height = String::new();

                println!("Enter the radius of the cylinder:");
                io::stdin().read_line(&mut radius).expect("Failed to read line");
                let radius: f64 = radius.trim().parse().expect("Invalid input");

                println!("Enter the height of the cylinder:");
                io::stdin().read_line(&mut height).expect("Failed to read line");
                let height: f64 = height.trim().parse().expect("Invalid input");

                let volume = volume_of_cylinder(radius, height);
                println!("The volume of the cylinder is: {}", volume);
            },
            6 => {
                println!("Exiting the program.");
                break;
            },
            _ => println!("Invalid choice, please select a number between 1 and 6."),
        }
    }
}
