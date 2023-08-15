use std::{fs::File, io::Read, collections::HashMap, cmp::max};
use plotters::prelude::*;
use std::process::Command;

pub enum Nucleotide {
    T,
    C,
    G,
    A
}

pub struct SequencingRead {
    pub bases: Vec<Nucleotide>,
    pub qualities: Vec<u8>
}

impl SequencingRead {
    pub fn new() -> Self {
        SequencingRead {
            bases: vec![],
            qualities: vec![]
        }
    }
}

pub struct QualityComputeEngine {
    // resulting_qualities: Vec<u8>,
    pub qualities: Box<HashMap<usize, Vec<u8>>>,
    max_length: usize
}

impl QualityComputeEngine {

    pub fn new() -> Self {
        QualityComputeEngine {
            qualities: Box::new(HashMap::new()),
            max_length: 0
        }
    }

    pub fn get_qualities(&self) -> Vec<f64> {
        let mut result = vec![0f64; self.max_length + 1];
        for length in self.qualities.keys().into_iter() {
            let s: u64 = self.qualities[length].iter().map(|x| *x as u64).sum();
            let n: usize = self.qualities[length].len();
            result[*length] = (s as f64) / (n as f64);

        };
        result
    }

    pub fn feed(&mut self, length: usize, quality: u8) {
        match self.qualities.get_mut(&length) {
            Some(length_qualities) => { length_qualities.push(quality); },
            None => { 
                self.qualities.insert(length, vec![quality]); 
                self.max_length = max(self.max_length, length);
            }
        }
    }

}

pub struct FASTQ {
    file_path: &'static str,

    // list of individual reads and qualities
    pub reads: Vec<SequencingRead>,

    // a single quality score for various read lengts averaged over all of the reads
    quality_compute: QualityComputeEngine
}

impl FASTQ {

    pub fn new(path: &'static str) -> Self {
        FASTQ { 
            file_path: path, 
            reads: vec![],
            quality_compute: QualityComputeEngine::new(),
        }
    }
    pub fn get_qualities(&self) -> Vec<f64> {
        self.quality_compute.get_qualities()
    }

    pub fn read_lines(&mut self) -> std::io::Result<()> {
        let mut file = File::open(self.file_path)?;
        let total_bytes: usize = file.metadata()?.len() as usize;
        println!("The input file is {:?} bytes long.", total_bytes);

        // TODO: split the reading in multiple readers based on CPU count

        // the current line we're reading in this batch
        let mut line_idx: usize = 0;

        // add initial placeholder for the reads
        self.reads.push(SequencingRead::new());

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
            
            for i in 0..bytes_read {
                let &char = &buffer[i];
                match char {
                    b'\n' => {
                        line_idx += 1;
                        base_pair_idx = 0;

                        // cycle the row counts to avoid expensive mod op
                        if line_idx == 4 { 
                            line_idx = 0;

                            // we expect another row if this is not the last byte of the file
                            if byte_offset < total_bytes || i < bytes_read - 1 {
                                self.reads.push(SequencingRead::new());
                            }
                        }
                    },
                    _ => {
                        // we only care about the line with the quality info
                        // don't waste compute collecting and counting the rest of bytes
                        if line_idx == 3 { 
                            let current_length = self.reads.len();
                            self.reads[current_length - 1].qualities.push(char - 33);
                            self.quality_compute.feed(base_pair_idx, char - 33);
                            base_pair_idx += 1;
                        }
                    }
                };
            }

        }

        Ok(())
    }

    
    pub fn plot_line_chart(self, filename: &str) -> Result<(), Box<dyn std::error::Error>> {

        let data: Vec<(f64, f64)> = self.get_qualities().iter().enumerate().map(|(index, &value)| (index as f64, value)).collect();
        println!("DATA: {:?}", data);
        
        // Set up the drawing area
        let root = BitMapBackend::new(filename, (800, 600)).into_drawing_area();
        root.fill(&WHITE)?;

        // Create a chart context
        let mut chart = ChartBuilder::on(&root)
            .caption("Read Quality Plot", ("sans-serif", 30).into_font())
            .margin(10)
            .x_label_area_size(30)
            .y_label_area_size(30)
            .build_cartesian_2d(0.0..(data.len() as f64), 0.0..50.0).unwrap();

        // Create a line series
        chart
            .configure_mesh()
            // .disable_x_mesh()
            // .disable_y_mesh()
            .x_label_formatter(&|x| format!("{}", x))
            // .y_label_formatter(&|y| format!("{}", y))
            .draw()?;

        chart.draw_series(LineSeries::new(
            data,
            &RED,
        )).unwrap();

        // Open the image using the default system image viewer
        // Command::new("xdg-open").arg(filename).output().ok();

        Ok(())
    }

}
