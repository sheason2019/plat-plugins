use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use anyhow::anyhow;

use crate::bindings::wasi::http::types::Method;

use super::{context::HttpContext, Router};

type Handler = fn(HttpContext) -> anyhow::Result<()>;

#[derive(Debug, Clone)]
pub struct RouterBuilder {
    path: String,
    children: Vec<Arc<Mutex<RouterBuilder>>>,

    get_handler: Option<Handler>,
    post_handler: Option<Handler>,
    put_handler: Option<Handler>,
    patch_handler: Option<Handler>,
    delete_handler: Option<Handler>,
}

impl RouterBuilder {
    pub fn new() -> Self {
        RouterBuilder {
            path: "".to_string(),
            children: Vec::new(),
            get_handler: None,
            post_handler: None,
            put_handler: None,
            patch_handler: None,
            delete_handler: None,
        }
    }

    pub fn build(&self) -> Router {
        Router {
            builder: Arc::new(Mutex::new(self.clone())),
        }
    }

    pub fn get(&mut self, path: &str, handler: Handler) -> anyhow::Result<()> {
        self.add_handler(Method::Get, path, handler)
    }
    pub fn post(&mut self, path: &str, handler: Handler) -> anyhow::Result<()> {
        self.add_handler(Method::Post, path, handler)
    }
    pub fn put(&mut self, path: &str, handler: Handler) -> anyhow::Result<()> {
        self.add_handler(Method::Put, path, handler)
    }
    pub fn patch(&mut self, path: &str, handler: Handler) -> anyhow::Result<()> {
        self.add_handler(Method::Patch, path, handler)
    }
    pub fn delete(&mut self, path: &str, handler: Handler) -> anyhow::Result<()> {
        self.add_handler(Method::Delete, path, handler)
    }

    fn add_handler(&mut self, method: Method, path: &str, handler: Handler) -> anyhow::Result<()> {
        if path.starts_with("/") {
            return Err(anyhow!("路径声明不能以 / 开头"));
        }

        let path_collect: Vec<&str> = path.split("/").filter(|i| i.len() > 0).collect();
        let node = self.get_child_or_create(path_collect);
        if node.is_some() {
            let target = node.unwrap();
            let mut target = target.lock().unwrap();

            match method {
                Method::Get => target.get_handler = Some(handler),
                Method::Post => target.post_handler = Some(handler),
                Method::Put => target.put_handler = Some(handler),
                Method::Patch => target.patch_handler = Some(handler),
                Method::Delete => target.delete_handler = Some(handler),
                _ => (),
            };
        } else {
            match method {
                Method::Get => self.get_handler = Some(handler),
                Method::Post => self.post_handler = Some(handler),
                Method::Put => self.put_handler = Some(handler),
                Method::Patch => self.patch_handler = Some(handler),
                Method::Delete => self.delete_handler = Some(handler),
                _ => (),
            };
        }

        Ok(())
    }

    pub fn get_handler(&self, method: Method) -> Option<Handler> {
        match method {
            Method::Get => self.get_handler,
            Method::Post => self.post_handler,
            Method::Put => self.put_handler,
            Method::Patch => self.patch_handler,
            Method::Delete => self.delete_handler,
            _ => None,
        }
    }

    fn get_child_or_create(&mut self, path: Vec<&str>) -> Option<Arc<Mutex<Self>>> {
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
                let mut new_node = RouterBuilder::new();
                new_node.path = p.to_string();

                let new_node = Arc::new(Mutex::new(new_node));
                self.children.push(new_node.clone());
                node = Some(new_node);
            }

            data = node;
        }

        data
    }

    pub fn match_router(
        this: Arc<Mutex<RouterBuilder>>,
        path_map: Arc<Mutex<HashMap<String, String>>>,
        path_collect: Vec<&str>,
    ) -> Option<Arc<Mutex<RouterBuilder>>> {
        let original_path = path_collect.join("/");
        let mut path_collect = path_collect.clone();
        if path_collect.len() == 0 {
            return Some(this);
        }
        let current_path = path_collect.remove(0);

        // 完全匹配路径，优先级最高
        let mut exact_routes: Vec<Arc<Mutex<RouterBuilder>>> = Vec::new();
        // 动态匹配路径，优先级中等
        let mut dynamic_routes: Vec<Arc<Mutex<RouterBuilder>>> = Vec::new();
        // 通配符路径，优先级最低
        let mut wildcard_routes: Vec<Arc<Mutex<RouterBuilder>>> = Vec::new();

        let router_builder = this.lock().unwrap();

        for child in router_builder.children.iter() {
            let p = { child.lock().unwrap().path.clone() };
            if p.starts_with(":") {
                dynamic_routes.push(child.clone());
                let path_name = &p.as_str()[1..];
                path_map
                    .lock()
                    .unwrap()
                    .insert(path_name.to_string(), router_builder.path.clone());

                continue;
            }
            if p.starts_with("*") {
                wildcard_routes.push(child.clone());
                let path_name = &p.as_str()[1..];
                path_map
                    .lock()
                    .unwrap()
                    .insert(path_name.to_string(), original_path.clone());
                continue;
            }
            if p == current_path {
                exact_routes.push(child.clone());
                continue;
            }
        }

        let next_routes = [exact_routes, dynamic_routes].concat();
        for route in next_routes {
            let match_route =
                RouterBuilder::match_router(route, path_map.clone(), path_collect.clone());
            if match_route.is_some() {
                return match_route;
            }
        }

        for route in wildcard_routes {
            return Some(route);
        }

        None
    }
}
