mod args;

use args::FlexiCsvArgs;
use clap::Parser;

fn main() {
    let args = FlexiCsvArgs::parse();
    println!("{:#?}", args);
}
