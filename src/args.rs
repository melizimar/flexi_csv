use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct FlexiCsvArgs {
    #[clap(subcommand)]
    pub operations_types: OperationsTypes,
}

#[derive(Debug, Subcommand)]
pub enum OperationsTypes {
    /// Slice -> Divide o arquivo CSV em novos arquivos menores
    Slice(SliceCommand),

    // Transform -> Aplica transformações aos campos do arquivo CSV
    //Transform(TransformCommand),
}

#[derive(Debug, Args)]
pub struct SliceCommand{
    #[clap(subcommand)]
    pub command: SliceSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum SliceSubcommand{
    Split(SplitCSV)
}

#[derive(Debug, Args)]
pub struct SplitCSV {
    pub input_path: PathBuf,
    pub output_dir: PathBuf
}