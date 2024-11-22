fn main() {
    let fullname = "Pan-Atlantic University"; 

    println!("Name: {}", fullname); 

    println!("Before trim:");
    println!("Length is {}", fullname.len()); 

    println!(); 

    println!("After trim:");
    println!("Length is {}", fullname.trim().len()); 
}

