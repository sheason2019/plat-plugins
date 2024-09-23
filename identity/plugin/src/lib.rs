#[allow(warnings)]
mod bindings;
mod express;
mod identity;
mod typings;
mod utils;

use std::env;

use bindings::wasi::http::types::{IncomingRequest, ResponseOutparam};
use express::{app::App, router::Router};
use identity::model::Identity;
use serde_json::json;

struct Component;

impl bindings::exports::wasi::http::incoming_handler::Guest for Component {
    fn handle(request: IncomingRequest, response_out: ResponseOutparam) {
        let mut app = App::new();
        app.layer(|ctx| {
            let mut router = Router::new();
            router.get("/api/check", |ctx, p| {
                let public_key = env::var("daemon_public_key").unwrap();
                let cur_user = Identity::find_by_public_key(public_key.clone());

                ctx.json(
                    200,
                    &json!({
                        "should_init": cur_user.is_none(),
                        "public_key": public_key,
                    }),
                );
                Ok(())
            });
            router.put("/api/current", |ctx, _| {
                let body = ctx.body();
                let mut identity: Identity =
                    serde_json::from_slice(&body).expect("序列化 Identity 信息失败");
                identity.public_key = env::var("daemon_public_key").unwrap();
                identity.save();

                ctx.text(200, "OK".to_string());
                Ok(())
            });

            router.handle(ctx)
        });

        app.handle(request, response_out).unwrap();
    }
}

impl bindings::exports::lifecycle::Guest for Component {
    fn on_start() {}
}

impl bindings::exports::task::Guest for Component {
    fn on_spawn(_payload: String) {}
}

bindings::export!(Component with_types_in bindings);
