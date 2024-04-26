use std::path::{Path, PathBuf};

use clap::{command, Parser, Subcommand};
use enum_dispatch::enum_dispatch;

use crate::CmdExector;

pub use self::base64::{Base64DecodeOpts, Base64EncodeOpts};
pub use self::csv::CsvOpts;
pub use self::genpass::GenPassOpts;
pub use self::http::HttpServeOpts;
pub use self::text::TextSubCommand;
pub use self::text::{TextKeyGenerateOpts, TextSignOpts, TextVerifyOpts};

mod base64;
mod csv;
mod genpass;
mod http;
mod text;

pub use self::base64::{Base64Format, Base64SubCommand};
pub use self::csv::OutputFormat;
pub use self::http::HttpSubCommand;
pub use self::text::TextSignFormat;

#[derive(Parser, Debug)]
#[command(name = "rcli", version, author, about, long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

impl CmdExector for Opts {
    async fn execute(self) -> anyhow::Result<()> {
        self.cmd.execute().await
    }
}

#[derive(Subcommand, Debug)]
#[enum_dispatch(CmdExector)]
pub enum SubCommand {
    #[command(name = "csv", about = "Show CSV, or convert CSV to other formats")]
    Csv(CsvOpts),
    #[command(name = "genpass", about = "Generate a random password")]
    GenPass(GenPassOpts),
    #[command(subcommand, about = "Base64 encode or decode")]
    Base64(Base64SubCommand),
    #[command(subcommand, about = "Text sign or verify")]
    Text(TextSubCommand),
    #[command(subcommand, about = "HTTP server")]
    Http(HttpSubCommand),
}

fn verify_file(filename: &str) -> Result<String, &'static str> {
    if filename == "-" || Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("File does not exist")
    }
}

fn verify_path(path: &str) -> Result<PathBuf, &'static str> {
    let p = Path::new(path);
    if p.exists() && p.is_dir() {
        Ok(path.into())
    } else {
        Err("Path does not exist or is not a directory")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_input_file() {
        assert_eq!(verify_file("-"), Ok("-".into()));
        assert_eq!(verify_file("Cargo.toml"), Ok("Cargo.toml".into()));
        assert_eq!(verify_file("not-exist"), Err("File does not exist"));
    }
}
