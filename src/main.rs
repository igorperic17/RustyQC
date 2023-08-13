pub mod fastq;

fn main() {
    let file = "sample_input.fastq";
    match fastq::read_lines(file) { 
        Err(err) => eprintln!("Error: {}", err),
        Ok(res) => {
            for line in res {
                println!("{:}", line);
            }
        }
    }
}
