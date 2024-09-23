#[allow(warnings)]
mod bindings;
mod store;
mod typings;
mod utils;

use bindings::wasi::http::types::{IncomingRequest, ResponseOutparam};

struct Component;

impl bindings::exports::wasi::http::incoming_handler::Guest for Component {
    fn handle(request: IncomingRequest, response_out: ResponseOutparam) {}
}

impl bindings::exports::lifecycle::Guest for Component {
    fn on_start() {}
}

impl bindings::exports::task::Guest for Component {
    fn on_spawn(_payload: String) {}
}

bindings::export!(Component with_types_in bindings);
