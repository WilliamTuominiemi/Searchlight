use walkdir::WalkDir;

use std::io::{self, Write};

fn main() {
    print!("Please enter your keyword: ");
    io::stdout().flush().unwrap(); 

    let mut search_keyword = String::new();
    io::stdin()
        .read_line(&mut search_keyword)
        .expect("Failed to read line");

    let search_keyword = search_keyword.trim(); 

    let start_dir = "/";

    for entry in WalkDir::new(start_dir).into_iter().filter_map(|e| e.ok()) {
        let file_name = entry.file_name().to_string_lossy();

        if file_name.contains(search_keyword) {
            println!("{}", entry.path().display());
        }
    }
}
