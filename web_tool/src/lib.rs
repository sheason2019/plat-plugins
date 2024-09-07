#[allow(warnings)]
mod bindings;
mod router;

use bindings::Guest;

struct Component;

impl bindings::exports::wasi::http::incoming_handler::Guest for Component {
    fn handle(
        _request: bindings::exports::wasi::http::incoming_handler::IncomingRequest,
        _response_out: bindings::exports::wasi::http::incoming_handler::ResponseOutparam,
    ) {
        todo!()
    }
}

impl Guest for Component {
    fn on_init() {
        todo!()
    }
}

bindings::export!(Component with_types_in bindings);
