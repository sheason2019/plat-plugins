use std::{collections::HashMap, path::PathBuf};

use crate::bindings::wasi::http::types::{ResponseOutparam, StatusCode};

use super::response::Response;

pub struct HttpContext {
    query_map: HashMap<String, String>,
    param_map: HashMap<String, String>,
    pub response: Response,
}

impl HttpContext {
    pub fn new(
        query_map: HashMap<String, String>,
        param_map: HashMap<String, String>,
        response_out: ResponseOutparam,
    ) -> Self {
        HttpContext {
            query_map,
            param_map,
            response: Response::new(response_out),
        }
    }

    pub fn send_file(self, file_path: PathBuf) -> anyhow::Result<()> {
        self.response.send_file(file_path)
    }

    pub fn send_json(self, status_code: StatusCode, json_string: String) -> anyhow::Result<()> {
        self.response.send_json(status_code, json_string)
    }

    pub fn send_bytes(self, status_code: StatusCode, bytes: Vec<u8>) -> anyhow::Result<()> {
        self.response.send_bytes(status_code, bytes)
    }

    pub fn query(&self, query_name: &str) -> Option<&String> {
        self.query_map.get(query_name)
    }

    pub fn params(&self, param_name: &str) -> Option<&String> {
        self.param_map.get(param_name)
    }
}
