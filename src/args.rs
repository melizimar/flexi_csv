use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct FlexiCsvArgs {
    // Divide o arquivo CSV em partes menores a escolha do usuário
    pub splitter: String
}
