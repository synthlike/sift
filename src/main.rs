use clap::Parser;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

mod ast;
mod lexer;
mod output;
mod parser;
mod selector;

use crate::lexer::Lexer;
use crate::output::FunctionOutput;
use crate::output::output_json;
use crate::output::output_tsv;
use crate::parser::Parser as SolidityParser;

#[derive(Parser)]
#[command(name = "sift")]
struct Cli {
    #[arg(value_name = "PATH")]
    path: PathBuf,

    #[arg(short, long)]
    json: bool,
}

fn main() {
    let cli = Cli::parse();

    let files = if cli.path.is_dir() {
        find_solidity_files(&cli.path)
    } else if cli.path.is_file() {
        vec![cli.path.clone()]
    } else {
        eprintln!("error: path does not exist: {}", cli.path.display());
        std::process::exit(1);
    };

    let mut all_functions = Vec::new();

    for file in files {
        match extract_functions_from_file(&file) {
            Ok(mut functions) => {
                functions.retain(|f| f.visibility == "external" || f.visibility == "public");
                all_functions.extend(functions);
            }
            Err(e) => {
                eprintln!("warning: failed to parse {}: {}", file.display(), e);
            }
        }
    }

    if cli.json {
        if let Err(e) = output_json(&all_functions) {
            eprintln!("error formatting JSON: {}", e);
            std::process::exit(1);
        }
    } else {
        output_tsv(&all_functions)
    }
}

fn find_solidity_files(dir: &Path) -> Vec<PathBuf> {
    WalkDir::new(dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().is_some_and(|ext| ext == "sol"))
        .map(|e| e.path().to_path_buf())
        .collect()
}

fn extract_functions_from_file(
    path: &Path,
) -> Result<Vec<FunctionOutput>, Box<dyn std::error::Error>> {
    let source = std::fs::read_to_string(path)?;

    let mut lexer = Lexer::new(&source);
    let tokens = lexer.tokenize();

    let mut parser = SolidityParser::new(tokens);
    let functions = parser.parse_all_functions();

    let output: Vec<FunctionOutput> = functions
        .iter()
        .map(|f| FunctionOutput::from_function(f))
        .collect();

    Ok(output)
}
