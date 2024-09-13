use std::sync::{Arc, Mutex};

use path_tree::PathTree;

use crate::bindings::wasi::http::types::Method;

pub struct Router {
    tree: PathTree<Arc<Mutex<RouteHandlerCollection>>>,
}

pub struct RouteHandlerCollection {
    get: Option<RouteHandler>,
    post: Option<RouteHandler>,
    put: Option<RouteHandler>,
    delete: Option<RouteHandler>,
    patch: Option<RouteHandler>,
}

pub type RouteHandler = fn(&mut super::context::Context, path_tree::Path) -> anyhow::Result<()>;

impl Router {
    pub fn new() -> Self {
        Router {
            tree: PathTree::new(),
        }
    }

    pub fn handle(&self, ctx: &mut super::context::Context) -> anyhow::Result<()> {
        let path_with_query = ctx.request.path_with_query().unwrap();

        let (collection, p) = match self.tree.find(&path_with_query) {
            None => return Ok(()),
            Some(value) => value,
        };

        match collection.lock().unwrap().find(ctx.request.method()) {
            None => return Ok(()),
            Some(handler) => {
                handler(ctx, p)?;
            }
        }

        Ok(())
    }

    pub fn get(&mut self, p: &'static str, handler: RouteHandler) {
        self.add_route(p, Method::Get, handler)
    }

    pub fn post(&mut self, p: &'static str, handler: RouteHandler) {
        self.add_route(p, Method::Post, handler)
    }

    pub fn put(&mut self, p: &'static str, handler: RouteHandler) {
        self.add_route(p, Method::Put, handler)
    }

    pub fn delete(&mut self, p: &'static str, handler: RouteHandler) {
        self.add_route(p, Method::Delete, handler)
    }

    pub fn patch(&mut self, p: &'static str, handler: RouteHandler) {
        self.add_route(p, Method::Patch, handler)
    }

    fn add_route(&mut self, p: &'static str, method: Method, handler: RouteHandler) {
        match self.tree.find(p) {
            Some((collection, _p)) => {
                collection.lock().unwrap().append(method, handler);
            }
            None => {
                let mut collection = RouteHandlerCollection::new();
                collection.append(method, handler);

                let _ = self.tree.insert(&p, Arc::new(Mutex::new(collection)));
            }
        }
    }
}

impl RouteHandlerCollection {
    pub const fn new() -> Self {
        RouteHandlerCollection {
            get: None,
            post: None,
            put: None,
            delete: None,
            patch: None,
        }
    }

    pub fn find(&self, method: Method) -> Option<RouteHandler> {
        match method {
            Method::Get => self.get,
            Method::Post => self.post,
            Method::Put => self.put,
            Method::Delete => self.delete,
            Method::Patch => self.patch,
            _ => None,
        }
    }

    pub fn append(&mut self, method: Method, handler: RouteHandler) {
        match method {
            Method::Get => self.get = Some(handler),
            Method::Post => self.post = Some(handler),
            Method::Put => self.put = Some(handler),
            Method::Delete => self.delete = Some(handler),
            Method::Patch => self.patch = Some(handler),
            _ => (),
        }
    }
}
