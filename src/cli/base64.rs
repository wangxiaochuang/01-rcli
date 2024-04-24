use clap::{arg, Parser, Subcommand, ValueEnum};

use super::verify_file;

#[derive(Debug, Subcommand)]
pub enum Base64SubCommand {
    #[command(name = "encode", about = "Encode to a base64 string")]
    Encode(Base64EncodeOpts),
    #[command(name = "decode", about = "Decode a base64 sgtring")]
    Decode(Base64DecodeOpts),
}

#[derive(Debug, Parser)]
pub struct Base64EncodeOpts {
    #[arg(short, long,  value_parser = verify_file, default_value = "-")]
    pub input: String,
    #[arg(value_enum, long, default_value = "standard")]
    pub format: Base64Format,
}

#[derive(Debug, Parser)]
pub struct Base64DecodeOpts {
    #[arg(short, long,  value_parser = verify_file, default_value = "-")]
    pub input: String,
    #[arg(value_enum, long, default_value = "standard")]
    pub format: Base64Format,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
#[value(rename_all = "lowercase")]
pub enum Base64Format {
    Standard,
    UrlSafe,
}
