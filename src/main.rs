mod args;
mod csv_utils;
mod threads;

use clap::Parser;

fn main() {
    let args = args::FlexiCsvArgs::parse();

    csv_utils::process_csv(args);
}
