use std::u64;

use crate::{
    bindings::wasi::{
        http::{
            outgoing_handler::{self, OutgoingRequest},
            types::{self, Fields, IncomingBody, OutgoingBody},
        },
        io::streams::StreamError,
    },
    typings::SignBox,
};
use serde_json::json;
use url::Position;

pub fn get_context() -> anyhow::Result<serde_json::Value> {
    let daemon_address = std::env::var("daemon_address")?;
    let daemon_address = url::Url::parse(&daemon_address)?;

    let req = OutgoingRequest::new(Fields::new());
    let path_with_query = &daemon_address[Position::BeforePath..];

    req.set_path_with_query(Some(path_with_query))
        .expect("set path with query");
    req.set_method(&types::Method::Get).expect("set method");
    req.set_scheme(Some(&types::Scheme::Http))
        .expect("set scheme failed");
    req.set_authority(Some(daemon_address.authority()))
        .expect("set authority failed");

    let future_response = outgoing_handler::handle(req, None)?;

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
    Ok(context)
}

pub fn get_sign_box(base64_url_data_string: String) -> anyhow::Result<SignBox> {
    let daemon_address = std::env::var("daemon_address")?;
    let daemon_address = url::Url::parse(&daemon_address)?.join("sign")?;

    let headers = Fields::new();
    headers.append(
        &"Content-Type".to_string(),
        &"application/json".as_bytes().to_vec(),
    )?;
    let req = OutgoingRequest::new(headers);
    let path_with_query = &daemon_address[Position::BeforePath..];

    req.set_path_with_query(Some(path_with_query))
        .expect("set path with query");
    req.set_method(&types::Method::Post).expect("set method");
    req.set_scheme(Some(&types::Scheme::Http))
        .expect("set scheme failed");
    req.set_authority(Some(daemon_address.authority()))
        .expect("set authority failed");

    {
        let body = req.body().unwrap();
        let data = json!({
            "base64_url_data_string": base64_url_data_string,
        });
        let data = serde_json::to_string(&data)?;
        let data = data.as_bytes();

        let body_len = data.len();
        let chunk_size = 2048;
        let mut chunk_count = body_len / 2048;
        if body_len % 2048 != 0 {
            chunk_count = chunk_count + 1;
        }

        let out = body.write().expect("outgoing stream");
        for i in 0..chunk_count {
            let mut end = (i + 1) * chunk_size;
            if end > body_len {
                end = body_len;
            }
            let chunk = &data[i * chunk_size..end];
            out.blocking_write_and_flush(chunk)?;
        }
        drop(out);

        OutgoingBody::finish(body, None).expect("finish request");
    }

    let future_response = outgoing_handler::handle(req, None)?;

    future_response.subscribe().block();

    let response = future_response
        .get()
        .expect("get future response failed")
        .expect("get incoming response result failed")
        .expect("get incoming response failed");

    let body = response.consume().expect("consume response failed");
    let body_bytes = read_incoming_body(body);
    let sign_box: SignBox = serde_json::from_slice(&body_bytes).expect("serilize failed");
    Ok(sign_box)
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
