use crate::args;

use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn process_csv(args: args::FlexiCsvArgs) {
    if let args::OperationsTypes::Transform(cmd) = args.operations_types {
        // Fluxo para TransformCommand
        println!("Executando TransformCommand no arquivo {:#?}", cmd);
    } else if let args::OperationsTypes::Slice(cmd) = args.operations_types {
        // Fluxo para SliceCommand
        println!("Executando SliceCommand no arquivo {:#?}", cmd);
    }
}

pub fn get_number_csv_files(
    number_lines_input_file: f64,
    number_lines_output_file: f64,
) -> Result<usize, Box<dyn Error>> {
    let result = number_lines_input_file / number_lines_output_file;
    Ok(result.ceil() as usize)
}

pub fn count_csv_lines<P>(path: P) -> io::Result<usize>
where
    P: AsRef<Path>,
{
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    let counter = reader.lines().count(); // Conta as linhas
    Ok(counter)
}
