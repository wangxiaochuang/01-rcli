use crate::{process_genpass, CmdExector};
use clap::{arg, Parser};
use zxcvbn::zxcvbn;

#[derive(Debug, Parser)]
pub struct GenPassOpts {
    #[arg(short, long, default_value_t = 16)]
    pub length: u8,

    #[arg(short, long, default_value_t = true)]
    pub uppercase: bool,

    #[arg(long, default_value_t = true)]
    pub lowercase: bool,

    #[arg(short, long, default_value_t = true)]
    pub number: bool,

    #[arg(short, long, default_value_t = true)]
    pub symbol: bool,
}

impl CmdExector for GenPassOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let password = process_genpass(
            self.length,
            self.uppercase,
            self.lowercase,
            self.number,
            self.symbol,
        )?;
        let score = zxcvbn(&password, &[])?;
        println!("{}", password);
        // 只打印到错误输出
        eprintln!("Password strength: {}", score.score());
        Ok(())
    }
}
