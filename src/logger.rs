use std::{fs::OpenOptions, io::Write};

use crate::log::Log;

#[derive(Clone)]
pub enum OutputTo {
    File(String),
    Console, 
}

#[derive(Clone)]
pub struct Logger {
    outputs: Vec<OutputTo>,
}

impl Logger {
    pub fn build(outputs: Vec<OutputTo>) -> Self {
        return Logger {
            outputs: outputs
        };
    }

    pub fn output(&self, log: Log) {
        for output in self.outputs.clone() {
            match output {
                OutputTo::File(filename) => {
                    let mut file = OpenOptions::new().append(true).create(true).open(filename).unwrap();
                    file.write(log.to_string_for_file().as_bytes()).expect("File input failed");
                },
                OutputTo::Console => {
                    println!("{}", log.to_string_for_console())
                },
            }
        }
    }

    pub fn output_in_expect(&self, log: Log) -> String {
        for output in self.outputs.clone() {
            match output {
                OutputTo::File(filename) => {
                    let mut file = OpenOptions::new().append(true).create(true).open(filename).unwrap();
                    file.write(log.to_string_for_file().as_bytes()).expect("File input failed");
                },
                OutputTo::Console => {},
            }
        }
        return log.to_string_for_console();
    }
}