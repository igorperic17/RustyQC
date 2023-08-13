use std::{fs::File, io::Read};

pub fn read_lines(path: &str) -> std::io::Result<Vec<String>> {
    let mut file = File::open(path)?;
    let total_bytes: usize = file.metadata()?.len() as usize;
    println!("The input file is {:?} bytes long.", total_bytes);

    // TODO: split the reading in multiple readers based on CPU count

    // let reader = BufReader::new(file);
    let mut lines = vec![String::new()];

    // the current line we're reading in this batch
    let mut line_idx: usize = 0;

    // while reading quality scores, this will be the index in the line
    // representing the current base index inside the read
    let mut base_pair_idx: usize = 0;
    
    // the current byte offset
    let mut byte_offset: usize = 0;

    // buffer in which we'll be reading file chunks
    let buffer_size: usize = 1024;
    let mut buffer: Vec<u8> = vec![0; buffer_size];
    while byte_offset < total_bytes {

        // got thought the buffer one byte at a time
        let bytes_read = file.read(&mut buffer).expect("Error reading file bytes into a buffer.");
        byte_offset += bytes_read;
        
        // file.read_exact(&mut buffer).expect("Error reading file bytes into a buffer.");
        for i in 0..bytes_read {
            let &char = &buffer[i];
            match char {
                b'\n' => {
                    line_idx += 1;
                    base_pair_idx = 0;
                    lines.push(String::new());
                },
                _ => {
                    lines[line_idx].push(char as char);
                    base_pair_idx += 1;
                }
            };
        }

    }

    Ok(lines)
}