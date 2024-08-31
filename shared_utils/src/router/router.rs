use std::collections::HashMap;

use crate::bindings::wasi::http::types::IncomingRequest;

use super::{HttpContext, RouterBuilder};

pub struct Router {
    pub builder: RouterBuilder,
}

impl Router {
    pub fn handle(&self, req: IncomingRequest) {}

    fn create_context(&self, req: IncomingRequest) -> HttpContext {
        let path_with_query = req.path_with_query().unwrap();
        let data: Vec<&str> = path_with_query.split("?").collect();

        let mut query_map: HashMap<String, String> = HashMap::new();
        if data.len() > 1 {
            let query_string_vec: Vec<&str> = data[1].split("&").collect();
            for query_string in query_string_vec {
                let query_string_data: Vec<&str> = query_string.split("=").collect();
                query_map.insert(
                    query_string_data[0].to_string(),
                    query_string_data[1].to_string(),
                );
            }
        }

        // 路由树得出应走的 router
        let mut path_map: HashMap<String, String> = HashMap::new();
        let path_collect: Vec<&str> = data[0].split("/").filter(|p| p.len() > 0).collect();

        HttpContext {
            path: data[0].to_string(),
            method: req.method(),
            query_map,
            path_map,
        }
    }

    fn walk_path() {
        // TODO: 
        // 1. 支持 * 路径匹配
        // 2. 返回 path_map 和 handler 
    }
}
