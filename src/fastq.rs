use std::io::{BufReader, BufRead};
use std::fs::File;

pub fn read_lines(path: &str) -> std::io::Result<Vec<String>> {
    let file = File::open(path)?;
    let total_bytes = file.metadata()?.len();
    println!("The input file is {:?} bytes long.", total_bytes);

    // TODO: split the reading in multiple readers based on CPU count

    let reader = BufReader::new(file);
    let mut lines = Vec::<String>::new();
    let mut line_idx: u64 = 0;
    for line in reader.lines() {
        line_idx += 1;

        if line_idx == 4 {
            lines.push(line?);
            line_idx = 0;
        }
    }

    Ok(lines)
}