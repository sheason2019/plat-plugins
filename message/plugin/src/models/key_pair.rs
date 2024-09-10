use base64::prelude::*;
use rand_core::OsRng;
use x25519_dalek::X25519_BASEPOINT_BYTES;

#[derive(Debug, Clone)]
pub struct X25519KeyPair {
    pub base64_url_public_key: String,
    pub base64_url_private_key: String,
}

impl X25519KeyPair {
    pub fn generate() -> Self {
        let secret = x25519_dalek::StaticSecret::random_from_rng(OsRng);
        let public_key = x25519_dalek::PublicKey::from(&secret);

        X25519KeyPair {
            base64_url_public_key: BASE64_URL_SAFE.encode(public_key.as_bytes()),
            base64_url_private_key: BASE64_URL_SAFE.encode(secret.as_bytes()),
        }
    }

    pub fn diffie_hellman(
        &self,
        base64_url_public_key: &String,
    ) -> anyhow::Result<x25519_dalek::SharedSecret> {
        let secret = BASE64_URL_SAFE.decode(&self.base64_url_private_key)?;
        let secret = x25519_dalek::StaticSecret::from(clone_key(secret));

        let public_key = BASE64_URL_SAFE.decode(base64_url_public_key)?;
        let public_key = x25519_dalek::PublicKey::from(clone_key(public_key));

        Ok(secret.diffie_hellman(&public_key))
    }
}

fn clone_key(value: Vec<u8>) -> [u8; 32] {
    let mut key_bytes = X25519_BASEPOINT_BYTES.clone();
    for i in 0..32 {
        key_bytes[i] = value[i];
    }

    key_bytes
}
