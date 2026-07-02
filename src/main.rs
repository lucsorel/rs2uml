mod domain;
mod parser;
mod walker;

use clap::{Parser, ValueEnum};
use std::path::PathBuf;
use walker::find_rust_files;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum ExportFormat {
    /// export the diagram in PlantUML syntax
    Plantuml,
    // /// export the diagram in Mermaid syntax
    // Mermaid,
}

fn assert_path_exists(pathname: &str) -> Result<PathBuf, String> {
    let path = PathBuf::from(pathname);
    if path.exists() {
        Ok(path)
    } else {
        Err(format!("File or folder '{}' does not exist", pathname))
    }
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)] // Read from `Cargo.toml`
struct Rs2UmlCli {
    /// Path to the folder or file to inspect
    #[arg(long,value_parser = assert_path_exists)]
    pub path: PathBuf,

    /// Diagram syntax
    #[arg(long, value_enum)]
    pub format: ExportFormat,
}

fn main() {
    let rs2uml_cli = Rs2UmlCli::parse();
    println!("{:?}", rs2uml_cli);
    let rust_files_result = find_rust_files(&rs2uml_cli.path);
    match rust_files_result {
        Ok(rust_files) => {
            for rust_file in rust_files {
                println!("{:?}", rust_file);
                let _ = parser::analyze_rust_ast(&rust_file);
            }
        }
        Err(error) => println!("{:?}", error),
    }
}
