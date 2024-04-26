use std::path::PathBuf;

use clap::{arg, Parser, Subcommand};
use enum_dispatch::enum_dispatch;

use crate::{process_http_serve, CmdExector};

use super::verify_path;

#[derive(Debug, Subcommand)]
#[enum_dispatch(CmdExector)]
pub enum HttpSubCommand {
    #[command(about = "Serve a directory over HTTP")]
    Serve(HttpServeOpts),
}

#[derive(Debug, Parser)]
pub struct HttpServeOpts {
    #[arg(short, long, value_parser = verify_path, default_value = ".")]
    pub dir: PathBuf,
    #[arg(short, long, default_value = "8080")]
    pub port: u16,
}

impl CmdExector for HttpServeOpts {
    async fn execute(self) -> anyhow::Result<()> {
        process_http_serve(self.dir.clone(), self.port).await
    }
}
