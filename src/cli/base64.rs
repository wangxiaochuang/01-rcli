use clap::{arg, Parser, Subcommand, ValueEnum};
use enum_dispatch::enum_dispatch;

use crate::{process_decode, process_encode, CmdExector};

use super::verify_file;

#[derive(Debug, Subcommand)]
#[enum_dispatch(CmdExector)]
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

impl CmdExector for Base64EncodeOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let encoded = process_encode(&self.input, self.format)?;
        println!("{}", encoded);
        Ok(())
    }
}

#[derive(Debug, Parser)]
pub struct Base64DecodeOpts {
    #[arg(short, long,  value_parser = verify_file, default_value = "-")]
    pub input: String,
    #[arg(value_enum, long, default_value = "standard")]
    pub format: Base64Format,
}

impl CmdExector for Base64DecodeOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let decoded = process_decode(&self.input, self.format)?;
        // TODO: assume decoded data is a string
        println!("{}", String::from_utf8_lossy(&decoded));
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, ValueEnum)]
#[value(rename_all = "lowercase")]
pub enum Base64Format {
    Standard,
    UrlSafe,
}
