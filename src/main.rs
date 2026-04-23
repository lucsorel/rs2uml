use clap::{Parser, ValueEnum};
use std::path::PathBuf;

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
struct Rs2umlCli {
    /// Path to the folder or file to inspect
    #[arg(long,value_parser = assert_path_exists)]
    pub path: PathBuf,

    /// Diagram syntax
    #[arg(long, value_enum)]
    pub format: ExportFormat,
}

fn main() {
    let rs2uml_cli = Rs2umlCli::parse();
    println!("{:?}", rs2uml_cli);
}
