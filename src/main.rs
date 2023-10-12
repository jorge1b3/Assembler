use std::env;
use std::fs::File;
use std::io::{self,BufRead,BufReader};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <file_name>", args[0]);
        std::process::exit(1);
    }

    let file_name = &args[1];
    
    let lines = parser(file_name)?;
    println!("{:?}", lines);
    Ok(())
}

// The parser read the file and return a vector of lines
fn parser(file_name: &str) -> io::Result<Vec<String>> {
    let file = File::open(file_name)?;
    let reader = BufReader::new(file);

    let mut lines: Vec<String> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let line = line.trim();

        if !line.is_empty() && !line.starts_with("//") {
            lines.push(line.to_string());
        }
    }

    Ok(lines)
}