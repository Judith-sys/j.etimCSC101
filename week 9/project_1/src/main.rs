use std::fs::File;
use std::io::{self, Write, Read};

fn main() -> io::Result<()> {
    // Step 1: Define the drink categories in a list format
    let waters_data = "Lager:\n\
                      - 33 Export\n\
                      - Desperados\n\
                      - Goldberg\n\
                      - Gulder\n\
                      - Heineken\n\
                      - Star\n\n\
                      Stout:\n\
                      - Legend\n\
                      - Turbo King\n\
                      - Williams\n\n\
                      Non-Alcoholic:\n\
                      - Maltina\n\
                      - Amstel Malta\n\
                      - Malta Gold\n\
                      - Fayrouz";

    // Step 2: Create and write to a file
    let mut file = File::create("waters_list.txt")?;
    file.write_all(waters_data.as_bytes())?;
    println!("Data has been written to 'waters_list.txt'.");

    // Step 3: Read from the file
    let mut file = File::open("waters_list.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    // Step 4: Display the contents
    println!("\nContents of 'waters_list.txt':\n");
    println!("{}", contents);

    Ok(())
}
