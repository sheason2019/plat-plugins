#[allow(warnings)]
mod bindings;
mod router;
mod store;
mod utils;

use std::path::Path;

use bindings::wasi::http::types::{IncomingRequest, ResponseOutparam};
use store::model::Identity;

struct Component;

impl bindings::exports::wasi::http::incoming_handler::Guest for Component {
    fn handle(request: IncomingRequest, response_out: ResponseOutparam) {
        let mut router_builder = router::RouterBuilder::new();
        router_builder
            .get("*path", |ctx| {
                let p = ctx.params("path").unwrap().clone();
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
        router_builder
            .get("api/identity/:id", |ctx| -> anyhow::Result<()> {
                let id = ctx.params("id");
                let identity = Identity::find_one(id.unwrap().clone())?;
                if identity.is_none() {
                    ctx.send_bytes(404, "Not Found".as_bytes().to_vec())?;
                    return Ok(());
                }

                let identity = identity.unwrap();
                let identity = serde_json::to_string(&identity)?;
                ctx.send_json(200, identity)?;

                Ok(())
            })
            .unwrap();
        router_builder
            .get("api/context", |ctx| -> anyhow::Result<()> {
                let context = utils::get_context();
                let context = serde_json::to_string(&context)?;
                ctx.send_json(200, context)?;

                Ok(())
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
        println!("开始执行身份插件逻辑");

        println!("尝试从守护进程获取上下文信息");
        let context = utils::get_context();
        println!("从守护进程获取上下文信息成功");
        let public_key = context
            .get("public_key")
            .expect("get public_key from context failed")
            .as_str()
            .expect("parse public_key as str failed");
        println!("当前用户公钥： {}", public_key);

        let identity =
            Identity::find_one(public_key.to_string()).expect("get identity by public_key failed");
        if identity.is_none() {
            println!("未找到当前用户的身份信息，即将自动初始化");
            let new_identity = Identity::create_empty(public_key.to_string());
            println!("初始化用户身份信息成功，正在保存");
            new_identity.save().expect("保存身份信息失败");
            println!("保存用户信息成功");
        }

        println!("身份插件初始化逻辑执行完毕");
    }
}

bindings::export!(Component with_types_in bindings);
