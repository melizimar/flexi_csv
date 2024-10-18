mod args;
mod csv_utils;
mod threads;

use clap::Parser;

use deunicode::deunicode;
use polars::prelude::*;
use std::fmt::Arguments;
use std::path::{Path, PathBuf};
use std::collections::HashMap;
use std::error::Error;
use std::fs::{create_dir_all, File};
use std::io::{self, BufRead};
use std::sync::Arc;
use std::sync::Mutex;
use std::{process, thread};

use indicatif::{ProgressBar, ProgressStyle};
use inflector::cases::titlecase::to_title_case;

fn main() {
    let arguments = args::FlexiCsvArgs::parse();

    if let args::OperationsTypes::Transform(cmd) = &arguments.operations_types {
        // Fluxo para TransformCommand
        println!("Executando TransformCommand no arquivo {:#?}", cmd);
    } 

    if let args::OperationsTypes::Slice(cmd) = &arguments.operations_types {
        // Fluxo para SliceCommand
        println!("Executando SliceCommand no arquivo {:#?}", cmd);
    }
}

struct CsvChunkReader<'a> {
    file_path: &'a PathBuf,
    skip_rows: usize,
    chunk_size: usize,
}

impl<'a> CsvChunkReader<'a> {
    // Função para inicializar o leitor de chunks
    pub fn new(file_path: &'a PathBuf, chunk_size: usize) -> Self {
        CsvChunkReader {
            file_path,
            skip_rows: 0, // Começa sem pular linhas
            chunk_size,
        }
    }

    // Função que retorna o próximo chunk de linhas como DataFrame
    pub fn next_chunk(&mut self) -> Result<DataFrame, PolarsError> {
        let lazy_df = LazyCsvReader::new(self.file_path)
            .with_has_header(true)
            .with_separator(b';')
            .with_truncate_ragged_lines(true)
            .with_ignore_errors(true) // Ignora erros de parsing
            .with_skip_rows_after_header(self.skip_rows)
            .with_n_rows(Some(self.chunk_size))
            .finish()?;

        // Atualiza o número de linhas que já foram lidas
        self.skip_rows += self.chunk_size;

        // Coleta o DataFrame
        lazy_df.collect()
    }
}

impl<'a> Iterator for CsvChunkReader<'a> {
    type Item = DataFrame;

    fn next(&mut self) -> Option<Self::Item> {
        match self.next_chunk() {
            Ok(df) => {
                if df.height() == 0 {
                    None // Quando não houver mais dados, retorna None
                } else {
                    Some(df) // Retorna o DataFrame
                }
            }
            Err(_) => None, // Em caso de erro, retorna None
        }
    }
}
