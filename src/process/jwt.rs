use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::{
    fs,
    path::Path,
    time::{SystemTime, UNIX_EPOCH},
};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    aud: String,
    iat: String,
    exp: u64,
}

pub fn process_jwt_sign(
    key: impl AsRef<Path>,
    sub: &str,
    aud: &str,
    duration: u64,
) -> anyhow::Result<String> {
    let key = fs::read_to_string(key)?;

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Failed to get timestamp")
        .as_secs();

    let claims = Claims {
        sub: sub.to_string(),
        aud: aud.to_string(),
        iat: now.to_string(),
        exp: now + duration,
    };
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(key.as_bytes()),
    )?;
    Ok(token)
}

pub fn process_jwt_verify<'a>(
    key: impl AsRef<Path>,
    allow_auds: impl AsRef<[&'a str]>,
    token: &str,
) -> anyhow::Result<bool> {
    let key = fs::read_to_string(key)?;
    let mut validation = Validation::new(Algorithm::HS256);
    validation.set_audience(allow_auds.as_ref());
    Ok(decode::<Claims>(token, &DecodingKey::from_secret(key.as_ref()), &validation).is_ok())
}
