mod parser;
mod schema;

use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <input_file>", args[0]);
        std::process::exit(1);
    }

    let path = Path::new(&args[1]);
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        match parser::parse_log_line(&line) {
            Ok(event) => println!("{}", serde_json::to_string_pretty(&event)?),
            Err(e) => eprintln!("Error parsing line: {}", e),
        }
    }

    Ok(())
}
