#[allow(warnings)]
mod bindings;
mod methods;
mod store;
mod utils;

use bindings::wasi::http::types::{IncomingRequest, Method, ResponseOutparam};
use utils::{http::send_file, http_context::HttpContext};

struct Component;

impl bindings::exports::wasi::http::incoming_handler::Guest for Component {
    fn handle(request: IncomingRequest, response_out: ResponseOutparam) {
        let http_ctx = HttpContext::from_request(request);

        match (http_ctx.method, http_ctx.path.clone()) {
            (Method::Get, p) => {
                let mut p = p;
                if p == "/" || p == "" {
                    p = "/index.html".to_string();
                }
                send_file(p, response_out)
            }
            (_method, _path) => panic!("Not Found"),
        }
        .expect("handle request failed");
    }
}

impl bindings::Guest for Component {
    fn on_init() {
        println!("hello init")
    }
}

bindings::export!(Component with_types_in bindings);
