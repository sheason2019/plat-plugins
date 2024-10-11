#[allow(warnings)]
mod bindings;
mod express;
mod identity;
mod typings;
mod utils;

use std::{env, path::Path};

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
            router.get("/api/check", |ctx, _| {
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
            router.get("/api/current", |ctx, _| {
                let identity =
                    match Identity::find_by_public_key(env::var("daemon_public_key").unwrap()) {
                        Some(value) => value,
                        None => {
                            ctx.json(200, &json!({"identity": None::<Identity>}));
                            return Ok(());
                        }
                    };

                ctx.json(200, &json!({"identity": &identity}));
                Ok(())
            });
            router.get("/:path*", |ctx, p| {
                let path_pair = p.params_iter().find(|i| i.0 == "path").unwrap();
                let asset_name = match path_pair.1.len() {
                    0 => "index.html",
                    _ => path_pair.1,
                };

                ctx.file(Path::new("/assets").join(asset_name));

                Ok(())
            });

            router.handle(ctx)
        });

        app.handle(request, response_out).unwrap();
    }
}

impl bindings::exports::lifecycle::Guest for Component {
    fn on_start() -> Result<(), String> {
        Ok(())
    }
}

impl bindings::exports::task::Guest for Component {
    fn on_spawn(_payload: String) -> Result<(), String> {
        Ok(())
    }
}

bindings::export!(Component with_types_in bindings);
