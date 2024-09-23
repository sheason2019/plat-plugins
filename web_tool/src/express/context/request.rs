use crate::bindings::wasi::{
    http::types::{IncomingBody, IncomingRequest, Method},
    io::streams::StreamError,
};

pub struct Request {
    pub incoming_request: IncomingRequest,
    body_bytes: Vec<u8>,
}

impl Request {
    pub fn new(incoming_request: IncomingRequest) -> Self {
        let body = incoming_request.consume().unwrap();
        Request {
            incoming_request,
            body_bytes: read_incoming_body(body),
        }
    }

    pub fn path_with_query(&self) -> Option<String> {
        self.incoming_request.path_with_query()
    }

    pub fn method(&self) -> Method {
        self.incoming_request.method()
    }

    pub fn body(&self) -> &Vec<u8> {
        &self.body_bytes
    }
}

fn read_incoming_body(body: IncomingBody) -> Vec<u8> {
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
