#[allow(warnings)]
mod bindings;
mod methods;
mod store;
mod utils;

use bindings::wasi::http::types::{
    Fields, IncomingRequest, OutgoingBody, OutgoingResponse, ResponseOutparam,
};

struct Component;

impl bindings::exports::wasi::http::incoming_handler::Guest for Component {
    fn handle(request: IncomingRequest, response_out: ResponseOutparam) {
        let hdrs = Fields::new();
        hdrs.append(
            &"Conetnt-Type".to_string(),
            &"text/html; charset=utf8".as_bytes().to_vec(),
        )
        .expect("append content type failed");

        let resp = OutgoingResponse::new(hdrs);
        let body = resp.body().expect("outgoing response");

        ResponseOutparam::set(response_out, Ok(resp));

        let out = body.write().expect("outgoing stream");
        out.blocking_write_and_flush(b"<h1>Hello world</h1>\n")
            .expect("writing response");
        drop(out);
        OutgoingBody::finish(body, None).unwrap();
    }
}

impl bindings::Guest for Component {
    fn on_init() {
        println!("hello init")
    }
}

bindings::export!(Component with_types_in bindings);
