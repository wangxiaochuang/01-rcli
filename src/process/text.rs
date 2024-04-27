use std::{fs, io::Read, path::Path};

use base64::{engine::general_purpose::STANDARD_NO_PAD, Engine};
use chacha20poly1305::{
    aead::{Aead, KeyInit},
    ChaCha20Poly1305, Nonce,
};
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use rand::rngs::OsRng;

use crate::{cli::TextSignFormat, get_reader, process_genpass};

pub trait TextSign {
    fn sign(&self, reader: &mut dyn Read) -> anyhow::Result<Vec<u8>>;
}

pub trait TextVerify {
    fn verify(&self, reader: impl Read, sig: &[u8]) -> anyhow::Result<bool>;
}

pub trait TextCipher {
    fn encrypt(&self, reader: impl Read) -> anyhow::Result<String>;
    fn decrypt(&self, reader: impl Read) -> anyhow::Result<String>;
}

pub trait KeyLoader {
    fn load(path: impl AsRef<Path>) -> anyhow::Result<Self>
    where
        Self: Sized;
}

pub trait KeyGenerate {
    fn generate() -> anyhow::Result<Vec<Vec<u8>>>;
}

pub struct Blake3 {
    key: [u8; 32],
}

pub struct Ed25519Signer {
    key: SigningKey,
}

pub struct Ed25519Verifier {
    key: VerifyingKey,
}

pub struct Chacha20poly1305Cipher {
    key: [u8; 32],
    nonce: [u8; 12],
}

pub fn process_text_sign(input: &str, key: &str, format: TextSignFormat) -> anyhow::Result<String> {
    let mut reader = get_reader(input)?;
    let signed = match format {
        TextSignFormat::Blake3 => {
            let signer = Blake3::load(key)?;
            signer.sign(reader.by_ref())?
        }
        TextSignFormat::Ed25519 => {
            let signer = Ed25519Signer::load(key)?;
            signer.sign(reader.by_ref())?
        }
    };
    let signed = STANDARD_NO_PAD.encode(signed);

    Ok(signed)
}

pub fn process_text_verify(
    input: &str,
    key: &str,
    format: TextSignFormat,
    sig: &str,
) -> anyhow::Result<bool> {
    let mut reader = get_reader(input)?;
    let sig = STANDARD_NO_PAD.decode(sig)?;
    let verified = match format {
        TextSignFormat::Blake3 => {
            let verifier = Blake3::load(key)?;
            verifier.verify(reader.by_ref(), &sig)?
        }
        TextSignFormat::Ed25519 => {
            let verifier = Ed25519Verifier::load(key)?;
            verifier.verify(reader.by_ref(), &sig)?
        }
    };

    Ok(verified)
}

pub fn process_text_generate(format: TextSignFormat) -> anyhow::Result<Vec<Vec<u8>>> {
    match format {
        TextSignFormat::Blake3 => Blake3::generate(),
        TextSignFormat::Ed25519 => Ed25519Signer::generate(),
    }
}

pub fn process_text_encrypt(input: &str, key: &str) -> anyhow::Result<String> {
    let mut reader = get_reader(input)?;
    let cipher = Chacha20poly1305Cipher::load(key)?;
    cipher.encrypt(reader.by_ref())
}

pub fn process_text_decrypt(input: &str, key: &str) -> anyhow::Result<String> {
    let mut reader = get_reader(input)?;
    let cipher = Chacha20poly1305Cipher::load(key)?;
    cipher.decrypt(&mut reader)
}

impl KeyLoader for Blake3 {
    fn load(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        let key = fs::read_to_string(path)?;
        let key = key.trim();
        Blake3::try_new(key.as_bytes())
    }
}

impl Blake3 {
    fn new(key: [u8; 32]) -> Self {
        Self { key }
    }

    fn try_new(key: &[u8]) -> anyhow::Result<Self> {
        Ok(Self::new(key.try_into().unwrap()))
    }
}

impl TextSign for Blake3 {
    fn sign(&self, reader: &mut dyn Read) -> anyhow::Result<Vec<u8>> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        Ok(blake3::keyed_hash(&self.key, &buf).as_bytes().to_vec())
    }
}

impl TextVerify for Blake3 {
    fn verify(&self, mut reader: impl Read, sig: &[u8]) -> anyhow::Result<bool> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let hash = blake3::keyed_hash(&self.key, &buf);
        let hash = hash.as_bytes();
        Ok(hash == sig)
    }
}

impl TextCipher for Chacha20poly1305Cipher {
    fn encrypt(&self, mut reader: impl Read) -> anyhow::Result<String> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let nonce = Nonce::from_slice(&self.nonce);
        let cipher = ChaCha20Poly1305::new_from_slice(&self.key)?;
        cipher
            .encrypt(nonce, buf.as_ref())
            .map(|cipher| STANDARD_NO_PAD.encode(cipher))
            .map_err(|e| anyhow::anyhow!(e))
    }

    fn decrypt(&self, mut reader: impl Read) -> anyhow::Result<String> {
        let mut encrypted = String::new();
        reader.read_to_string(&mut encrypted)?;
        let encrypted = STANDARD_NO_PAD.decode(encrypted)?;

        let nonce = Nonce::from_slice(&self.nonce);
        let cipher = ChaCha20Poly1305::new_from_slice(&self.key)?;
        cipher
            .decrypt(nonce, encrypted.as_ref())
            .map(|plain| String::from_utf8(plain).unwrap())
            .map_err(|e| anyhow::anyhow!(e))
    }
}

impl KeyLoader for Chacha20poly1305Cipher {
    fn load(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        let key = fs::read_to_string(path)?;
        let key = key.trim();
        let nonce = blake3::hash(key.as_bytes()).as_bytes().to_vec();
        Chacha20poly1305Cipher::try_new(key.as_bytes(), &nonce[..12])
    }
}

impl Chacha20poly1305Cipher {
    fn new(key: [u8; 32], nonce: [u8; 12]) -> Self {
        Self { key, nonce }
    }

    fn try_new(key: &[u8], nonce: &[u8]) -> anyhow::Result<Self> {
        Ok(Self::new(
            key.try_into().unwrap(),
            nonce.try_into().unwrap(),
        ))
    }
}

impl KeyLoader for Ed25519Signer {
    fn load(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        let key = fs::read(path)?;
        Self::try_new(&key)
    }
}

impl KeyLoader for Ed25519Verifier {
    fn load(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        let key = fs::read(path)?;
        Self::try_new(&key)
    }
}

impl KeyGenerate for Blake3 {
    fn generate() -> anyhow::Result<Vec<Vec<u8>>> {
        let key = process_genpass(32, true, true, true, true)?;
        let key = key.as_bytes().to_vec();
        Ok(vec![key])
    }
}

impl KeyGenerate for Ed25519Signer {
    fn generate() -> anyhow::Result<Vec<Vec<u8>>> {
        let mut csprng = OsRng;
        let sk = SigningKey::generate(&mut csprng);
        let pk = sk.verifying_key().to_bytes().to_vec();
        let sk = sk.to_bytes().to_vec();
        Ok(vec![sk, pk])
    }
}

impl Ed25519Signer {
    fn new(key: SigningKey) -> Self {
        Self { key }
    }

    fn try_new(key: &[u8]) -> anyhow::Result<Self> {
        Ok(Self::new(SigningKey::from_bytes(key.try_into().unwrap())))
    }
}

impl Ed25519Verifier {
    pub fn new(key: VerifyingKey) -> Self {
        Self { key }
    }

    pub fn try_new(key: &[u8]) -> anyhow::Result<Self> {
        Ok(Self::new(
            VerifyingKey::from_bytes(key.try_into().unwrap()).unwrap(),
        ))
    }
}

impl TextSign for Ed25519Signer {
    fn sign(&self, reader: &mut dyn Read) -> anyhow::Result<Vec<u8>> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        Ok(self.key.sign(&buf).to_bytes().to_vec())
    }
}

impl TextVerify for Ed25519Verifier {
    fn verify(&self, mut reader: impl Read, sig: &[u8]) -> anyhow::Result<bool> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let sig = Signature::from_bytes(sig.try_into().unwrap());
        Ok(self.key.verify(&buf, &sig).is_ok())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blake3_sign_verify() -> anyhow::Result<()> {
        let key = [0u8; 32];
        let blake3 = Blake3::try_new(&key)?;
        let data = b"hello world";
        let sig = blake3.sign(&mut &data[..])?;

        assert!(blake3.verify(&data[..], &sig)?);

        Ok(())
    }

    #[test]
    fn test_ed25519_sign_verify() -> anyhow::Result<()> {
        let sk = Ed25519Signer::load("fixtures/ed25519.sk")?;
        let pk = Ed25519Verifier::load("fixtures/ed25519.pk")?;

        let data = b"hello world";
        let sig = sk.sign(&mut &data[..])?;

        assert!(pk.verify(&data[..], &sig)?);

        Ok(())
    }
}
