use std::fs::File;
use std::io::{self, Write};

fn main() -> io::Result<()> {
    
    let students = vec![
        ("Oluthi Mordi", "ACC10211111", "Accounting", 300),
        ("Adams Aliyu", "EC010110101", "Economics", 100),
        ("Shania Bolade", "C5C10528825", "Computer", 200),
        ("Adekunle Gold", "FEE11000202", "Electrical", 200),
        ("Blanca Ldemoh", "MEE10202001", "Mechanical", 100),
    ];

    println!("PAU SMIS\n");
    println!("Student\n");

    for student in &students {
        let (name, matric_number, department, level) = student;
        println!("Name: {}", name);
        println!("Matric. Number: {}", matric_number);
        println!("Department: {}", department);
        println!("Level: {}", level);
        println!();  
    }

    let file_name = "students.txt";
    let mut file = File::create(file_name)?;

    file.write_all(b"PAU SMIS\n\nStudent\n\n")?;

    for student in &students {
        let (name, matric_number, department, level) = student;
        let student_info = format!(
            "{}\n{}\n{}\n{}\n\n",
            name, matric_number, department, level
        );
        file.write_all(student_info.as_bytes())?;
    }

    println!("Student details have been saved to '{}'.", file_name);

    Ok(())
}