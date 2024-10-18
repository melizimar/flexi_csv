use crate::args;

pub fn process_csv(args: args::FlexiCsvArgs) {
    if let args::OperationsTypes::Transform(cmd) = args.operations_types {
        // Fluxo para TransformCommand
        println!("Executando TransformCommand no arquivo {:#?}", cmd);
    } else if let args::OperationsTypes::Slice(cmd) = args.operations_types {
        // Fluxo para SliceCommand
        println!("Executando SliceCommand no arquivo {:#?}", cmd);
    }
}
