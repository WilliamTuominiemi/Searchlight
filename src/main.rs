use std::io::{self, Write};

fn main() {
    print!("Enter your search term: ");
    io::stdout().flush().unwrap(); 

    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");

    let name = name.trim();
    println!("Query: {}", name);
}
