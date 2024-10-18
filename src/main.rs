mod args;
mod threads;

use args::FlexiCsvArgs;
use threads::get_num_threads;
use clap::Parser;

fn main() {
    let args = FlexiCsvArgs::parse();
    let num = get_num_threads();

    println!("{:#?}", num);
    println!("{:#?}", args);
  
}
