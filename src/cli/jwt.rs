use clap::{arg, Parser, Subcommand};
use enum_dispatch::enum_dispatch;

use crate::{process_jwt_sign, process_jwt_verify, CmdExector};

use super::{parse_duration, verify_file};

#[derive(Debug, Subcommand)]
#[enum_dispatch(CmdExector)]
pub enum JwtSubCommand {
    #[command(about = "Sign a jwt")]
    Sign(JwtSignOpts),
    #[command(about = "Verify a jwt")]
    Verify(JwtVerifyOpts),
}

#[derive(Debug, Parser)]
pub struct JwtSignOpts {
    #[arg(short, long, value_parser = verify_file)]
    pub key: String,
    #[arg(short, long)]
    pub sub: String,
    #[arg(short, long)]
    pub aud: String,
    #[arg(short, long, value_parser = parse_duration)]
    pub exp: u64,
}
impl CmdExector for JwtSignOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let sig = process_jwt_sign(&self.key, &self.sub, &self.aud, self.exp)?;
        println!("{}", sig);
        Ok(())
    }
}

#[derive(Debug, Parser)]
pub struct JwtVerifyOpts {
    #[arg(short, long, value_parser = verify_file)]
    pub key: String,
    #[arg(long, help = "set allowed auds eg: 123,456")]
    pub allow_auds: String,
    #[arg(short, long, help = "specific jwt token to verify")]
    pub token: String,
}

impl CmdExector for JwtVerifyOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let allow_auds: Vec<&str> = self.allow_auds.split(',').collect();
        let verified = process_jwt_verify(&self.key, allow_auds, &self.token)?;
        println!("{}", verified);
        Ok(())
    }
}
