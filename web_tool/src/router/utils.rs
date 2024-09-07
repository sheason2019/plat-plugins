use std::u64;

use crate::bindings::wasi::{http::types::IncomingBody, io::streams::StreamError};

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
