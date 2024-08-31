use std::sync::{Arc, Mutex};

use anyhow::anyhow;

use crate::bindings::wasi::http::types::Method;

use super::{context::HttpContext, Router};

type Handler = fn(HttpContext) -> anyhow::Result<()>;

#[derive(Debug, Clone)]
pub struct RouterBuilder {
    path: String,
    children: Vec<Arc<Mutex<RouterBuilder>>>,

    handlers: Vec<(Method, Handler)>,
}

impl RouterBuilder {
    pub fn new() -> Self {
        RouterBuilder {
            path: "".to_string(),
            children: Vec::new(),
            handlers: Vec::new(),
        }
    }

    pub fn build(&self) -> Router {
        Router {
            builder: self.clone(),
        }
    }

    pub fn get(&mut self, path: String, handler: Handler) -> anyhow::Result<()> {
        self.add_handler(Method::Get, path, handler)
    }
    pub fn post(&mut self, path: String, handler: Handler) -> anyhow::Result<()> {
        self.add_handler(Method::Post, path, handler)
    }
    pub fn put(&mut self, path: String, handler: Handler) -> anyhow::Result<()> {
        self.add_handler(Method::Put, path, handler)
    }
    pub fn patch(&mut self, path: String, handler: Handler) -> anyhow::Result<()> {
        self.add_handler(Method::Patch, path, handler)
    }
    pub fn delete(&mut self, path: String, handler: Handler) -> anyhow::Result<()> {
        self.add_handler(Method::Delete, path, handler)
    }

    fn add_handler(
        &mut self,
        method: Method,
        path: String,
        handler: Handler,
    ) -> anyhow::Result<()> {
        if path.starts_with("/") {
            return Err(anyhow!("路径声明不能以 / 开头"));
        }

        // 分解传入的 path，如果 collect 长度为 0
        // 则表示操作的目标是当前节点，否则向下寻找节点
        let path_collect: Vec<&str> = path.split("/").filter(|i| i.len() > 0).collect();
        let node = self.get_child_or_create(path_collect);

        node.lock().unwrap().handlers.push((method, handler));

        Ok(())
    }

    fn get_child_or_create(&mut self, path: Vec<&str>) -> Arc<Mutex<Self>> {
        let mut data: Option<Arc<Mutex<Self>>> = None;
        for p in path {
            let mut node: Option<Arc<Mutex<Self>>> = None;
            for child in self.children.iter() {
                if child.lock().unwrap().path == p {
                    node = Some(child.clone());
                    break;
                }
            }

            if node.is_none() {
                let new_node = RouterBuilder {
                    path: p.to_string(),
                    children: Vec::new(),
                    handlers: Vec::new(),
                };
                let new_node = Arc::new(Mutex::new(new_node));
                self.children.push(new_node.clone());
                node = Some(new_node);
            }

            data = node;
        }

        data.unwrap()
    }
}
