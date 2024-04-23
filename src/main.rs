use anyhow::Result;
use clap::Parser;
use rcli::{
    cli::{Base64SubCommand, Opts, SubCommand},
    process_csv, process_decode, process_encode, process_genpass,
};

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
        SubCommand::GenPass(ref opts) => process_genpass(
            opts.length,
            opts.uppercase,
            opts.lowercase,
            opts.number,
            opts.symbol,
        )?,
        SubCommand::Base64(ref cmd) => match cmd {
            Base64SubCommand::Encode(ref opts) => process_encode(&opts.input, opts.format)?,
            Base64SubCommand::Decode(ref opts) => process_decode(&opts.input, opts.format)?,
        },
    };

    Ok(())
}
