use std::fs;

fn main() {
    fs::remove_file("jd.txt").expect("could not remove file");
    println!("File is removed")
}