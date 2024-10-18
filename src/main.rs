mod args;
mod threads;

use args::{FlexiCsvArgs, OperationsTypes};
use clap::Parser;

fn main() {
    let args = FlexiCsvArgs::parse();

    process_csv(args);
}

fn process_csv(args: FlexiCsvArgs) {
    if let OperationsTypes::Transform(cmd) = args.operations_types {
        // Fluxo para TransformCommand
        println!("Executando TransformCommand no arquivo {:#?}", cmd);
    } else if let OperationsTypes::Slice(cmd) = args.operations_types {
        // Fluxo para SliceCommand
        println!("Executando SliceCommand no arquivo {:#?}", cmd);
    }
}
