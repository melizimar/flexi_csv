mod args;
mod csv_utils;
mod threads;

use clap::Parser;
use csv_utils::{count_csv_lines, get_number_csv_files};

use deunicode::deunicode;
use polars::prelude::*;
use std::collections::HashMap;
use std::error::Error;
use std::fmt::Arguments;
use std::fs::{create_dir_all, File};
use std::io::{self, BufRead};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::Mutex;
use std::{process, thread};

use indicatif::{ProgressBar, ProgressStyle};
use inflector::cases::titlecase::to_title_case;

fn main() -> Result<(), Box<dyn Error>> {
    let arguments = args::FlexiCsvArgs::parse();

    if let args::OperationsTypes::Slice(args) = arguments.operations_types.clone() {
        // Abre o arquivo
        let metadata = std::fs::metadata(args.input_file.clone())?;

        // Verifica se o arquivo existe
        if !args.input_file.exists() {
            println!("O arquivo não existe, por gentileza informe um arquivo valido.");
            process::exit(1);
        } else if metadata.len() == 0 {
            println!("O arquivo está vazio, por gentileza informe um arquivo valido.");
            process::exit(1);
        }

        // Cria o diretório de saída se não existir
        create_dir_all(args.output_dir.clone())?;

        let input_file = args.input_file.clone();

        // let mut transformations: HashMap<String, Vec<String>> = HashMap::new();

        // if let Some(vec) = &args.to_uppercase {
        //     transformations.insert("to_uppercase".to_string(), vec.clone());
        // }
        // if let Some(vec) = &args.to_lowercase {
        //     transformations.insert("to_lowercase".to_string(), vec.clone());
        // }
        // if let Some(vec) = &args.to_normalized {
        //     transformations.insert("to_normalized".to_string(), vec.clone());
        // }
        // if let Some(vec) = &args.to_titlecase {
        //     transformations.insert("to_titlecase".to_string(), vec.clone());
        // }

        let num_lines_input_file = count_csv_lines(&input_file).unwrap();
        let chunck_size = args.num_lines_output_file * 20;
        let num_csv_files = get_number_csv_files(
            num_lines_input_file as f64,
            args.num_lines_output_file as f64,
        )
        .unwrap();

        let dataframes = CsvChunkReader::new(&args.input_file, chunck_size); // 100.000 linhas por chunk

        let indexes_file_names: Vec<usize> = (1..1 + num_csv_files).collect();

        let shared_indexes = Arc::new(Mutex::new(indexes_file_names));

        // Criar barra de progresso
        let progress_bar = ProgressBar::new(num_csv_files as u64);
        progress_bar.set_style(
        ProgressStyle::default_bar()
            .template(
                "Processando arquivos...{pos}/{len}\n[{elapsed_precise}] [{wide_bar}] ({percent}%) ",
            )
            .unwrap(),
    );

        for mut df in dataframes {
            // for (key, columns) in transformations.clone() {
            //     if key == *"to_uppercase" {
            //         for column in &columns {
            //             let uppercase_column = df
            //                 .lazy() // Usa lazy execution
            //                 .with_column(col(column).str().to_uppercase())
            //                 .collect()?;
            //             df = uppercase_column;
            //         }
            //     }
            //     if key == *"to_lowercase" {
            //         for column in &columns {
            //             let lowercase_column = df
            //                 .lazy() // Usa lazy execution
            //                 .with_column(col(column).str().to_lowercase())
            //                 .collect()?;
            //             df = lowercase_column;
            //         }
            //     }
            //     if key == *"to_normalized" {
            //         for column in &columns {
            //             let col_series = df.column(column)?.str()?;
            //             // Remova acentos de cada valor na série

            //             let no_accents: Vec<Option<String>> = col_series
            //                 .into_iter()
            //                 .map(|opt_s| opt_s.map(deunicode)) // Remove acentos
            //                 .collect();

            //             // Cria uma nova série com os valores sem acentos
            //             let new_series = Series::new(column.into(), no_accents);

            //             // Substitui a coluna antiga pela nova no DataFrame
            //             df.replace(column, new_series)?;
            //         }
            //     }
            //     if key == *"to_titlecase" {
            //         for column in &columns {
            //             let col_series = df.column(column)?.str()?;

            //             // Remova acentos de cada valor na série
            //             let no_accents: Vec<Option<String>> = col_series
            //                 .into_iter()
            //                 .map(|opt_s| opt_s.map(to_title_case)) // Remove acentos
            //                 .collect();

            //             // Cria uma nova série com os valores sem acentos
            //             let new_series = Series::new(column.into(), no_accents);

            //             // Substitui a coluna antiga pela nova no DataFrame
            //             df.replace(column, new_series)?;
            //         }
            //     }
            // }

            // Número de linhas por arquivo
            let chunk_size = args.num_lines_output_file;

            // Total de arquivos que vamos gerar
            let total_chunks = get_number_csv_files(df.height() as f64, chunk_size as f64).unwrap();

            // Um vetor para armazenar as threads
            let mut handles = vec![];

            // Usamos Arc e Mutex para compartilhar o DataFrame entre as threads
            let df = Arc::new(df);

            // Criar as threads
            for i in 0..total_chunks {
                // Clonar o Arc DF para cada thread tenha acesso ao mesmo DF
                let df = Arc::clone(&df);

                // Clonar o Arc para que cada thread tenha acesso ao mesmo vetor
                let shared_indexes = Arc::clone(&shared_indexes);

                // Caminho para o diretorio de output
                let output_dir: String = args.output_dir.clone().to_str().unwrap().to_string();

                // Obtém o nome do arquivo sem a extensão
                let file_name = if let Some(file_stem) = input_file.file_stem() {
                    file_stem.to_str().unwrap_or("").to_string() // Converte para String
                } else {
                    String::new() // Retorna uma String vazia se não conseguir
                };

                let handle = thread::spawn(move || {
                    let start = i * chunk_size;
                    let end = ((i + 1) * chunk_size).min(df.height());
                    let mut chunk = df.slice(start as i64, end - start);

                    let index_value = {
                        let mut data = shared_indexes.lock().unwrap();

                        // Verifica se há elementos no vetor
                        if data.is_empty() {
                            None // Retorna None se o vetor estiver vazio
                        } else {
                            // Remove e retorna o primeiro valor
                            Some(data.remove(0))
                        }
                    };

                    // Criar o nome do arquivo
                    let output_file_name = format!(
                        "{}/{}-{}.csv",
                        &output_dir,
                        file_name.clone(),
                        index_value.unwrap()
                    );

                    let mut file = File::create(&output_file_name).unwrap();

                    // Gravar o DataFrame no arquivo
                    CsvWriter::new(&mut file)
                        .include_header(true)
                        .with_separator(args.delimiter as u8)
                        .finish(&mut chunk)
                        .unwrap();
                });

                handles.push(handle);
            }

            // Aguardar a conclusão de todas as threads
            for handle in handles {
                let _ = handle.join();
                progress_bar.inc(1);
            }
        }

        progress_bar.finish_with_message("Todos os arquivos foram processados.");
    }

    if let args::OperationsTypes::Transform(args) = arguments.operations_types.clone() {
        // Fluxo para TransformCommand
        // Abre o arquivo
        let metadata = std::fs::metadata(args.input_file.clone())?;

        let progress_bar = ProgressBar::new(metadata.len() as u64);
        progress_bar.set_style(
        ProgressStyle::default_bar()
            .template(
                "Processando arquivo...{pos}/{len}\n[{elapsed_precise}] [{wide_bar}] ({percent}%)",
            )
            .unwrap(),
        );

        let mut lazy_df = LazyCsvReader::new(args.input_file)
            .with_has_header(true)
            .with_separator(b';')
            .with_truncate_ragged_lines(true)
            .with_ignore_errors(true) // Ignora erros de parsing
            //.with_skip_rows_after_header(self.skip_rows)
            //.with_n_rows(Some(self.chunk_size))
            .finish()?;

        let mut transformations: HashMap<String, Vec<String>> = HashMap::new();

        if let Some(vec) = &args.to_uppercase {
            transformations.insert("to_uppercase".to_string(), vec.clone());
        }
        if let Some(vec) = &args.to_lowercase {
            transformations.insert("to_lowercase".to_string(), vec.clone());
        }
        if let Some(vec) = &args.to_normalized {
            transformations.insert("to_normalized".to_string(), vec.clone());
        }
        if let Some(vec) = &args.to_titlecase {
            transformations.insert("to_titlecase".to_string(), vec.clone());
        }

        for (key, columns) in transformations.clone() {
            if key == *"to_uppercase" {
                for column in &columns {
                    let uppercase_column = lazy_df
                        .with_column(col(column).str().to_uppercase());
                    lazy_df = uppercase_column;
                }
            }
            if key == *"to_lowercase" {
                for column in &columns {
                    let lowercase_column = lazy_df
                        .lazy() // Usa lazy execution
                        .with_column(col(column).str().to_lowercase());
                    lazy_df = lowercase_column;
                }
            }
            
        //     if key == *"to_normalized" {
        //         for column in &columns {
        //             let col_series = lazy_df.column(column)?.str()?;
        //             // Remova acentos de cada valor na série

        //             let no_accents: Vec<Option<String>> = col_series
        //                 .into_iter()
        //                 .map(|opt_s| opt_s.map(deunicode)) // Remove acentos
        //                 .collect();

        //             // Cria uma nova série com os valores sem acentos
        //             let new_series = Series::new(column.into(), no_accents);

        //             // Substitui a coluna antiga pela nova no DataFrame
        //             lazy_df.replace(column, new_series)?;
        //         }
        //     }
        //     if key == *"to_titlecase" {
        //         for column in &columns {
        //             let col_series = df.column(column)?.str()?;

        //             // Remova acentos de cada valor na série
        //             let no_accents: Vec<Option<String>> = col_series
        //                 .into_iter()
        //                 .map(|opt_s| opt_s.map(to_title_case)) // Remove acentos
        //                 .collect();

        //             // Cria uma nova série com os valores sem acentos
        //             let new_series = Series::new(column.into(), no_accents);

        //             // Substitui a coluna antiga pela nova no DataFrame
        //             df.replace(column, new_series)?;
        //         }
        //     }
         }

         let mut df = lazy_df.clone().collect()?;
        
         let mut file = File::create("./input/test.csv").unwrap();

         // Gravar o DataFrame no arquivo
         CsvWriter::new(&mut file)
             .include_header(true)
             .with_separator(args.delimiter as u8)
             .finish(&mut df)
             .unwrap();
        
        for i in 1..metadata.len(){
            progress_bar.inc(1);
        }
        progress_bar.finish_with_message("Arquivos processado.");
    }

    Ok(())
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
