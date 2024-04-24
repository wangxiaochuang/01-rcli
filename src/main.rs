use std::fs;

use anyhow::Result;
use clap::Parser;
use rcli::{
    cli::{Base64SubCommand, Opts, SubCommand, TextSignFormat, TextSubCommand},
    process_csv, process_decode, process_encode, process_genpass, process_text_generate,
    process_text_sign, process_text_verify,
};
use zxcvbn::zxcvbn;

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
        SubCommand::GenPass(ref opts) => {
            let password = process_genpass(
                opts.length,
                opts.uppercase,
                opts.lowercase,
                opts.number,
                opts.symbol,
            )?;
            let score = zxcvbn(&password, &[])?;
            // 只打印到错误输出
            eprintln!("Password strength: {}", score.score());
        }
        SubCommand::Base64(cmd) => match cmd {
            Base64SubCommand::Encode(ref opts) => {
                let encoded = process_encode(&opts.input, opts.format)?;
                println!("{}", encoded);
            }
            Base64SubCommand::Decode(ref opts) => {
                let decoded = process_decode(&opts.input, opts.format)?;
                // TODO: assume decoded data is a string
                println!("{}", String::from_utf8_lossy(&decoded));
            }
        },
        SubCommand::Text(cmd) => match cmd {
            TextSubCommand::Sign(ref opts) => {
                let sig = process_text_sign(&opts.input, &opts.key, opts.format)?;
                println!("{}", sig);
            }
            TextSubCommand::Verify(ref opts) => {
                let verified = process_text_verify(&opts.input, &opts.key, opts.format, &opts.sig)?;
                println!("{}", verified);
            }
            TextSubCommand::Generate(ref opts) => {
                let key = process_text_generate(opts.format)?;
                match opts.format {
                    TextSignFormat::Blake3 => {
                        let name = opts.output.join("blake3.txt");
                        fs::write(name, &key[0])?;
                    }
                    TextSignFormat::Ed25519 => {
                        let name = &opts.output;
                        fs::write(name.join("ed25519.sk"), &key[0])?;
                        fs::write(name.join("ed25519.pk"), &key[1])?;
                    }
                }
            }
        },
    };

    Ok(())
}
