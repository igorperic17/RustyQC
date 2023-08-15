pub mod fastq;

use fastq::FASTQ;

fn main() {
    let file = "sample_input.fastq";
    let mut fastq_parser = FASTQ::new(file);
    match fastq_parser.read_lines() { 
        Err(err) => eprintln!("Error: {}", err),
        _ => {
            // for line in &fastq_parser.reads {
            //     println!("{:?}", line.qualities);
            // }

            println!("Resulting quality control lines: {}", fastq_parser.reads.len());
        }
    }
}
