use std::path::Path;

use express::router::Router;

#[allow(warnings)]
mod bindings;
mod express;

struct Component;

impl bindings::exports::wasi::http::incoming_handler::Guest for Component {
    fn handle(
        req: bindings::exports::wasi::http::incoming_handler::IncomingRequest,
        response_out: bindings::exports::wasi::http::incoming_handler::ResponseOutparam,
    ) {
        let mut app = express::app::App::new();
        app.layer(|ctx| {
            let mut router = Router::new();
            router.get("/:path*", |ctx, p| {
                println!("path params: {:?}", p.params());
                let path = p.params_iter().find(|i| i.0 == "path").unwrap();

                let assets_root = Path::new("/assets");
                ctx.file(assets_root.join(path.1));

                Ok(())
            });

            router.handle(ctx)
        });

        match app.handle(req, response_out) {
            Ok(_) => (),
            Err(e) => println!("handle application failed: {:?}", e),
        }
    }
}

impl bindings::exports::lifecycle::Guest for Component {
    fn on_start() {}
}

impl bindings::exports::task::Guest for Component {
    fn on_spawn(payload: String) {
        todo!()
    }
}

bindings::export!(Component with_types_in bindings);
