use anyhow::Result;
use rand::seq::SliceRandom;

const UPPER: &[u8] = b"ABCDEFGHJKLMNPQRSTUVWXYZ";
const LOWER: &[u8] = b"abcdefghijkmnopqrstuvwxyz";
const NUMBER: &[u8] = b"23456789";
const SYMBOL: &[u8] = b"!@#$%^&*_";
pub fn process_genpass(
    length: u8,
    upper: bool,
    lower: bool,
    number: bool,
    symbol: bool,
) -> Result<String> {
    let rng = &mut rand::thread_rng();
    let mut chars = Vec::new();
    let mut password = Vec::with_capacity(length as usize);

    if upper {
        chars.extend_from_slice(UPPER);
        // password.push(UPPER.choose(rng).map(|c| *c).unwrap());
        password.push(UPPER.choose(rng).copied().unwrap());
    }
    if lower {
        chars.extend_from_slice(LOWER);
        password.push(LOWER.choose(rng).copied().unwrap());
    }
    if number {
        chars.extend_from_slice(NUMBER);
        password.push(NUMBER.choose(rng).copied().unwrap());
    }
    if symbol {
        chars.extend_from_slice(SYMBOL);
        password.push(SYMBOL.choose(rng).copied().unwrap());
    }

    let mut remain = length as isize - password.len() as isize;
    if remain < 0 {
        remain = 0;
    }
    password.extend_from_slice(
        chars
            .choose_multiple(rng, remain as usize)
            .copied()
            .collect::<Vec<u8>>()
            .as_slice(),
    );

    password.shuffle(rng);
    let password = String::from_utf8(password)?;

    Ok(password)
}
