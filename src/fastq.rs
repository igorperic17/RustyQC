use std::io::{BufReader, BufRead};
use std::fs::File;

pub fn read_lines(path: &str) -> std::io::Result<Vec<String>> {
    let file = File::open(path)?;
    let total_bytes = file.metadata()?.len();
    println!("The input file is {:?} bytes long.", total_bytes);

    // TODO: split the reading in multiple readers based on CPU count

    let reader = BufReader::new(file);
    let mut lines = Vec::<String>::new();
    for line in reader.lines() {
        lines.push(line?);
    }

    Ok(lines)
}