use anyhow::Result;
use clap::Parser;
use rcli::{process_csv, Opts, SubCommand};

fn main() -> Result<()> {
    let opts = Opts::parse();
    match opts.cmd {
        SubCommand::Csv(ref opts) => {
            let output = if let Some(output) = &opts.output {
                output.clone()
            } else {
                format!("output.{}", opts.format)
            };
            process_csv(&opts.input, output, opts.format)?
        }
    };

    Ok(())
}
