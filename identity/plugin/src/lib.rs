#[allow(warnings)]
mod bindings;
mod router;
mod store;

use std::{path::Path, u64};

use bindings::wasi::{
    http::{
        outgoing_handler::{self, OutgoingRequest},
        types::{self, Fields, IncomingRequest, ResponseOutparam},
    },
    io::streams::StreamError,
};
use store::model::Identity;
use url::Position;

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
            .get("/api/identity/:id", |ctx| -> anyhow::Result<()> {
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
        let router = router_builder.build();
        router
            .handle(request, response_out)
            .expect("handle request failed");
    }
}

impl bindings::Guest for Component {
    fn on_init() {
        println!("开始执行身份插件逻辑");

        let daemon_address =
            std::env::var("daemon_address").expect("get env var daemon_address failed");
        println!("守护进程地址：{}", &daemon_address);

        let daemon_address =
            url::Url::parse(&daemon_address).expect("parse daemon_address to url failed");

        println!("开始从守护进程获取上下文信息");
        let mut body_bytes = Vec::new();
        {
            let req = OutgoingRequest::new(Fields::new());
            let context_url = daemon_address.join("context").unwrap();
            let path_with_query = &context_url[Position::BeforePath..];

            req.set_path_with_query(Some(path_with_query))
                .expect("set path with query");
            req.set_method(&types::Method::Get).expect("set method");
            req.set_scheme(Some(&types::Scheme::Http))
                .expect("set scheme failed");
            req.set_authority(Some(context_url.authority()))
                .expect("set authority failed");

            let future_response =
                outgoing_handler::handle(req, None).expect("handle request failed");

            future_response.subscribe().block();

            let response = future_response
                .get()
                .expect("get future response failed")
                .expect("get incoming response result failed")
                .expect("get incoming response failed");

            let body = response.consume().expect("consume response failed");
            let stream = body.stream().expect("get stream from body failed");
            loop {
                let mut chunk = match stream.read(u64::MAX) {
                    Ok(value) => value,
                    Err(StreamError::Closed) => break,
                    Err(e) => panic!("read stream error: {}", e),
                };

                body_bytes.append(&mut chunk);
            }
        }

        let context: serde_json::Value =
            serde_json::from_slice(&body_bytes).expect("serilize context failed");
        let public_key = context
            .get("public_key")
            .expect("get public_key from context failed")
            .as_str()
            .expect("parse public_key as str failed");
        println!("获取上下文信息成功，当前用户公钥： {}", public_key);

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
