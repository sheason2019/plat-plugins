use std::collections::HashMap;

use crate::bindings::wasi::http::types::{IncomingRequest, Method};

pub struct HttpContext {
    pub path: String,
    pub query: HashMap<String, String>,
    pub method: Method,
}

impl HttpContext {
    pub fn from_request(req: IncomingRequest) -> Self {
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

        HttpContext {
            path: data[0].to_string(),
            query: query_map,
            method: req.method(),
        }
    }
}
