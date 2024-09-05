use std::u64;

use crate::bindings::wasi::{
    http::{
        outgoing_handler::{self, OutgoingRequest},
        types::{self, Fields, IncomingBody},
    },
    io::streams::StreamError,
};
use url::Position;

pub fn get_context() -> serde_json::Value {
    let daemon_address =
        std::env::var("daemon_address").expect("get env var daemon_address failed");

    let daemon_address =
        url::Url::parse(&daemon_address).expect("parse daemon_address to url failed");

    let req = OutgoingRequest::new(Fields::new());
    let path_with_query = &daemon_address[Position::BeforePath..];

    req.set_path_with_query(Some(path_with_query))
        .expect("set path with query");
    req.set_method(&types::Method::Get).expect("set method");
    req.set_scheme(Some(&types::Scheme::Http))
        .expect("set scheme failed");
    req.set_authority(Some(daemon_address.authority()))
        .expect("set authority failed");

    let future_response = outgoing_handler::handle(req, None).expect("handle request failed");

    future_response.subscribe().block();

    let response = future_response
        .get()
        .expect("get future response failed")
        .expect("get incoming response result failed")
        .expect("get incoming response failed");

    let body = response.consume().expect("consume response failed");
    let body_bytes = read_incoming_body(body);

    let context: serde_json::Value =
        serde_json::from_slice(&body_bytes).expect("serilize context failed");
    context
}

pub fn read_incoming_body(body: IncomingBody) -> Vec<u8> {
    let mut body_bytes = Vec::new();
    let stream = body.stream().expect("get stream from body failed");
    loop {
        let mut chunk = match stream.read(u64::MAX) {
            Ok(value) => value,
            Err(StreamError::Closed) => break,
            Err(e) => panic!("read stream error: {}", e),
        };

        body_bytes.append(&mut chunk);
    }

    body_bytes
}
