use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
mod tokenization;
pub use tokenization::tokenize_python_code;

fn main() -> Result<(), std::io::Error> {
    // File hosts must exist in current path before this produces output
    let args: Vec<String> = env::args().collect();
    let path = &args[2];
    let out_path = &args[3];
    let tok_interval = 100000;

    println!("input file {}", &path);
    println!("output file {}", &out_path);

    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    let mut counter = 0;
    // Read the file line by line using the lines() iterator from std::io::BufRead.
    for maybe_line in reader.lines() {
        let tokens = maybe_line.map(tokenize_python_code);
        let n_line_tokens = tokens.map(|ts| ts.len()).unwrap_or(0);
        counter += n_line_tokens;
        // Show the line and its number.
        if (counter % tok_interval) == 0 {
            println!("{} M tokens", (counter as f32) / 1000000.0);
        };
    }
    Ok(())
}
