#[allow(warnings)]
mod bindings;
mod models;
mod router;

use base64::Engine;
use bindings::exports::wasi::http::incoming_handler::{IncomingRequest, ResponseOutparam};
use chacha20poly1305::{
    aead::{generic_array::GenericArray, Aead},
    AeadCore, ChaCha20Poly1305, KeyInit,
};

struct Component;

impl bindings::exports::wasi::http::incoming_handler::Guest for Component {
    fn handle(request: IncomingRequest, response_out: ResponseOutparam) {
        todo!()
    }
}

impl bindings::exports::lifecycle::Guest for Component {
    fn before_start() {
        let alice = models::key_pair::X25519KeyPair::generate();
        println!("alice x25519 keypair {:?}", alice);

        let bob = models::key_pair::X25519KeyPair::generate();
        println!("bob x25519 keypair {:?}", bob);

        let alice_shared_secret = alice
            .diffie_hellman(&bob.base64_url_public_key)
            .expect("alice shared_secret");
        let bob_shared_secret = bob
            .diffie_hellman(&alice.base64_url_public_key)
            .expect("bob shared_secret");

        println!(
            "alice shared secret: {}",
            base64::prelude::BASE64_URL_SAFE.encode(alice_shared_secret.as_bytes())
        );
        println!(
            "bob shared secret: {}",
            base64::prelude::BASE64_URL_SAFE.encode(bob_shared_secret.as_bytes())
        );

        assert_eq!(alice_shared_secret.as_bytes(), bob_shared_secret.as_bytes());

        let plaintext = b"Hello world";
        let (nonce, cipher_text) = {
            let key = GenericArray::from_slice(alice_shared_secret.as_bytes());
            let cipher = ChaCha20Poly1305::new(&key);
            let nonce = ChaCha20Poly1305::generate_nonce(&mut chacha20poly1305::aead::OsRng);
            let cipher_text = cipher
                .encrypt(&nonce, plaintext.as_ref())
                .expect("加密失败");
            println!(
                "cipher text is: {}",
                base64::prelude::BASE64_URL_SAFE.encode(&cipher_text)
            );
            (nonce, cipher_text)
        };

        let key = GenericArray::from_slice(bob_shared_secret.as_bytes());
        let cipher = ChaCha20Poly1305::new(&key);
        let decrypt_text = cipher
            .decrypt(&nonce, cipher_text.as_ref())
            .expect("decrypt failed");
        println!(
            "decrypt text is: {}",
            std::str::from_utf8(decrypt_text.as_ref()).unwrap()
        );
    }

    fn on_started() {
        println!("on started");
    }
}

bindings::export!(Component with_types_in bindings);
