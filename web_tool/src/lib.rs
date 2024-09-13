#[allow(warnings)]
mod bindings;
mod router;

struct Component;

impl bindings::exports::wasi::http::incoming_handler::Guest for Component {
    fn handle(
        _request: bindings::exports::wasi::http::incoming_handler::IncomingRequest,
        _response_out: bindings::exports::wasi::http::incoming_handler::ResponseOutparam,
    ) {
        todo!()
    }
}

impl bindings::exports::lifecycle::Guest for Component {
    fn before_start() {
        todo!()
    }

    fn on_started() {
        todo!()
    }
}

bindings::export!(Component with_types_in bindings);
