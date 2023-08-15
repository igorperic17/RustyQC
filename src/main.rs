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

            let qualities = fastq_parser.get_qualities();
            println!("MultiQC: {:?}", qualities);
            println!("MultiQC (count): {:?}", qualities.len());

            if let Err(err) = fastq_parser.plot_line_chart("plot.png") {
                println!("Error saving the plot: {}", err);
            }
        }
    }
}
