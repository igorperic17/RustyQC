pub mod fastq;

use fastq::FASTQ;

fn main() {
    let file = "sample_input.fastq";
    let mut fastq_parser = FASTQ::new(file);
    match fastq_parser.read_lines() { 
        Err(err) => eprintln!("Error: {}", err),
        Ok(res) => {
            for line in &res {
                println!("{}", line);
            }

            println!("Resulting quality control lines: {}", res.len());
        }
    }
}
