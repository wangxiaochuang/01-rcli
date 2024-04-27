use std::path::{Path, PathBuf};

use clap::{command, Parser, Subcommand};
use enum_dispatch::enum_dispatch;
use regex::Regex;

use crate::CmdExector;

pub use self::base64::{Base64DecodeOpts, Base64EncodeOpts};
pub use self::csv::CsvOpts;
pub use self::genpass::GenPassOpts;
pub use self::http::HttpServeOpts;
pub use self::jwt::{JwtSignOpts, JwtSubCommand, JwtVerifyOpts};
pub use self::text::{
    TextDecryptOpts, TextEncryptOpts, TextKeyGenerateOpts, TextSignOpts, TextSubCommand,
    TextVerifyOpts,
};

mod base64;
mod csv;
mod genpass;
mod http;
mod jwt;
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
    #[command(subcommand, about = "jwt sign or verify")]
    Jwt(JwtSubCommand),
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

fn parse_duration(duration: &str) -> Result<u64, &'static str> {
    let re = Regex::new(r"(\d+)([smhdw]?)").unwrap();
    if let Some(captures) = re.captures(duration) {
        let duration = captures.get(1).unwrap().as_str();
        let unit = captures.get(2).unwrap().as_str();
        let errf = |_| "invalid duration string";
        match unit {
            "" => duration.parse::<u64>().map_err(errf),
            "s" => duration.parse::<u64>().map_err(errf),
            "m" => duration.parse::<u64>().map(|x| x * 60).map_err(errf),
            "h" => duration.parse::<u64>().map(|x| x * 3600).map_err(errf),
            "d" => duration.parse::<u64>().map(|x| x * 3600 * 24).map_err(errf),
            "w" => duration
                .parse::<u64>()
                .map(|x| x * 3600 * 24 * 7)
                .map_err(errf),
            _ => Err("unsupported unit"),
        }
    } else {
        Err("invalid duration string")
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

    #[test]
    fn test_parse_duration() {
        assert_eq!(parse_duration("10"), Ok(10));
        assert_eq!(parse_duration("10s"), Ok(10));
        assert_eq!(parse_duration("10m"), Ok(600));
        assert_eq!(parse_duration("10h"), Ok(36000));
        assert_eq!(parse_duration("10d"), Ok(864000));
        assert_eq!(parse_duration("10w"), Ok(6048000));
    }
}
