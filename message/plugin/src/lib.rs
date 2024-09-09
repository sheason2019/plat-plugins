#[allow(warnings)]
mod bindings;
mod router;

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
    fn on_init() {}
}

bindings::export!(Component with_types_in bindings);
