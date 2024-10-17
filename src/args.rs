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
    /// Caminho para o arquivo de entrada (obrigatório)
    #[arg(short, long)]
    input_file: PathBuf,
    /// Caminho para o diretório de saída (obrigatório)
    #[arg(short, long)]
    output_dir: PathBuf,
    /// Número de linhas para cada arquivo de saída (obrigatório)
    #[arg(short, long)]
    num_lines_output_file: usize,
    /// Delimitador do arquivo CSV o padrão é ";"
    #[arg(short, long, default_value_t = ';')]
    delimiter: char,
    /// Número de Threads para criação dos arquivos. O valor padrão é definido de acordo com cada maquina
    #[arg(long, default_value_t = num_cpus::get())]
    num_threads: usize,
    /// Recebe o nome dos campos como argumento e transforma eles em UPPERCASE
    #[arg(long, num_args = 1..)]
    to_uppercase: Option<Vec<String>>,
    /// Recebe o nome dos campos como argumento e transforma eles em LOWERCASE
    #[arg(long, num_args = 1..)]
    to_lowercase: Option<Vec<String>>,
    /// Recebe o nome dos campos como argumento e transforma eles em NORMALIZED (sem acentuação)
    #[arg(long, num_args = 1..)]
    to_normalized: Option<Vec<String>>,
    /// Recebe o nome dos campos como argumento e transforma as informações em TITLE CASE
    #[arg(long, num_args = 1..)]
    to_titlecase: Option<Vec<String>>,
}
