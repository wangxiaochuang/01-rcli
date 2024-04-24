use std::fmt::Display;

use clap::{Parser, ValueEnum};

use super::verify_file;

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum OutputFormat {
    Json,
    Yaml,
}
impl Display for OutputFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let t = match self {
            OutputFormat::Json => "json",
            OutputFormat::Yaml => "yaml",
        };
        write!(f, "{}", t)
    }
}

#[derive(Debug, Parser)]
pub struct CsvOpts {
    #[arg(short, long, value_parser = verify_file)]
    pub input: String,

    #[arg(short, long)]
    pub output: Option<String>,

    #[arg(value_enum, long)]
    pub format: OutputFormat,

    #[arg(short, long, default_value_t = ',')]
    delimiter: char,

    #[arg(long, default_value_t = true)]
    header: bool,
}
