use std::path::PathBuf;

use clap::{arg, Parser, Subcommand, ValueEnum};
use enum_dispatch::enum_dispatch;
use tokio::fs;

use crate::{
    process_text_decrypt, process_text_encrypt, process_text_generate, process_text_sign,
    process_text_verify, CmdExector,
};

use super::{verify_file, verify_path};

#[derive(Debug, Subcommand)]
#[enum_dispatch(CmdExector)]
pub enum TextSubCommand {
    #[command(about = "Sign a message with a private/shared key")]
    Sign(TextSignOpts),
    #[command(about = "Verify a signed message")]
    Verify(TextVerifyOpts),
    #[command(about = "generate a new key")]
    Generate(TextKeyGenerateOpts),
    #[command(about = "encrypt a message by chacha20")]
    Encrypt(TextEncryptOpts),
    #[command(about = "decrypt a message by chacha20")]
    Decrypt(TextDecryptOpts),
}

#[derive(Debug, Parser)]
pub struct TextSignOpts {
    #[arg(short, long,  value_parser = verify_file, default_value = "-")]
    pub input: String,
    #[arg(short, long)]
    pub key: String,
    #[arg(long, default_value = "blake3")]
    pub format: TextSignFormat,
}
impl CmdExector for TextSignOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let sig = process_text_sign(&self.input, &self.key, self.format)?;
        println!("{}", sig);
        Ok(())
    }
}

#[derive(Debug, Parser)]
pub struct TextVerifyOpts {
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,
    #[arg(short, long)]
    pub key: String,
    #[arg(short, long)]
    pub sig: String,
    #[arg(long, default_value = "blake3")]
    pub format: TextSignFormat,
}

impl CmdExector for TextVerifyOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let verified = process_text_verify(&self.input, &self.key, self.format, &self.sig)?;
        println!("{}", verified);
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, ValueEnum)]
#[value(rename_all = "lowercase")]
pub enum TextSignFormat {
    Blake3,
    Ed25519,
}

#[derive(Debug, Parser)]
pub struct TextKeyGenerateOpts {
    #[arg(long, default_value = "blake3")]
    pub format: TextSignFormat,
    #[arg(short, long, value_parser = verify_path)]
    pub output: PathBuf,
}

impl CmdExector for TextKeyGenerateOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let key = process_text_generate(self.format)?;
        match self.format {
            TextSignFormat::Blake3 => {
                let name = self.output.join("blake3.txt");
                fs::write(name, &key[0]).await?;
            }
            TextSignFormat::Ed25519 => {
                let name = &self.output;
                fs::write(name.join("ed25519.sk"), &key[0]).await?;
                fs::write(name.join("ed25519.pk"), &key[1]).await?;
            }
        };
        Ok(())
    }
}

#[derive(Debug, Parser)]
pub struct TextEncryptOpts {
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,
    #[arg(short, long)]
    pub key: String,
}

impl CmdExector for TextEncryptOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let enc = process_text_encrypt(&self.input, &self.key)?;
        println!("{}", enc);
        Ok(())
    }
}

#[derive(Debug, Parser)]
pub struct TextDecryptOpts {
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,
    #[arg(short, long)]
    pub key: String,
}

impl CmdExector for TextDecryptOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let dec = process_text_decrypt(&self.input, &self.key)?;
        println!("{}", dec);
        Ok(())
    }
}
