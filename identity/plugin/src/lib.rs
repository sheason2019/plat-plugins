#[allow(warnings)]
mod bindings;
mod methods;
mod router;
mod store;

use std::path::Path;

use bindings::wasi::http::types::{IncomingRequest, ResponseOutparam};

struct Component;

impl bindings::exports::wasi::http::incoming_handler::Guest for Component {
    fn handle(request: IncomingRequest, response_out: ResponseOutparam) {
        let mut router_builder = router::RouterBuilder::new();
        router_builder
            .get("*path", |ctx| {
                let p = ctx.params("path".to_string()).unwrap().clone();
                let storage_path = Path::new("/storage");
                ctx.send_file(storage_path.join(p))
            })
            .unwrap();
        router_builder
            .get("", |ctx| {
                let storage_path = Path::new("/storage");
                ctx.send_file(storage_path.join("index.html"))
            })
            .unwrap();
        let router = router_builder.build();
        router
            .handle(request, response_out)
            .expect("handle request failed");
    }
}

impl bindings::Guest for Component {
    fn on_init() {
        println!("hello init")
    }
}

bindings::export!(Component with_types_in bindings);
