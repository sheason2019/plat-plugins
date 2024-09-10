#[allow(warnings)]
mod bindings;
mod models;
mod router;

use base64::Engine;
use bindings::{
    exports::wasi::http::incoming_handler::{IncomingRequest, ResponseOutparam},
    Guest,
};

struct Component;

impl bindings::exports::wasi::http::incoming_handler::Guest for Component {
    fn handle(request: IncomingRequest, response_out: ResponseOutparam) {
        todo!()
    }
}

impl Guest for Component {
    fn on_init() {
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
    }
}

bindings::export!(Component with_types_in bindings);
