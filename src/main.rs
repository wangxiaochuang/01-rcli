use anyhow::Result;
use clap::Parser;
use rcli::{cli::Opts, CmdExector};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let opts = Opts::parse();
    opts.execute().await
}
