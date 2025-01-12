use std::fs::File;
use std::io::{self, Write, Read};

fn main() -> io::Result<()> {
    // Step 1: Create a vector of tuples to store student details
    let students = vec![
        ("Oluchi Mordi", "ACC1021111", "Accounting", 100),
        ("Adams Aliyu", "ECO1011001", "Economics", 100),
        ("Shania Bolade", "CSC1038282", "Computer", 200),
        ("Adekunle Godi", "EEE1102020", "Electrical", 200),
        ("Blanca Edemoh", "MEE1020201", "Mechanical", 100),
    ];

    // Step 2: Save student details to a file
    let mut file = File::create("student_details.txt")?;
    for student in &students {
        let line = format!(
            "Name: {}\nMatric Number: {}\nDepartment: {}\nLevel: {}\n\n",
            student.0, student.1, student.2, student.3
        );
        file.write_all(line.as_bytes())?;
    }
    println!("Student details saved to 'student_details.txt'.");

    // Step 3: Read the file and display its content
    let mut file = File::open("student_details.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    println!("\nContents of 'student_details.txt':\n{}", contents);

    Ok(())
}
