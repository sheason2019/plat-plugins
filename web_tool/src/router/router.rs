use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use anyhow::Context;

use crate::bindings::wasi::http::types::{IncomingRequest, ResponseOutparam};

use super::{response::Response, HttpContext, RouterBuilder};

pub struct Router {
    pub builder: Arc<Mutex<RouterBuilder>>,
}

impl Router {
    pub fn handle(
        &self,
        req: IncomingRequest,
        response_out: ResponseOutparam,
    ) -> anyhow::Result<()> {
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
        let path_map: Arc<Mutex<HashMap<String, String>>> = Arc::new(Mutex::new(HashMap::new()));
        let path_collect: Vec<&str> = data[0].split("/").filter(|p| p.len() > 0).collect();
        let router_builder =
            RouterBuilder::match_router(self.builder.clone(), path_map.clone(), path_collect);
        if router_builder.is_none() {
            Response::new(response_out).send_bytes(404, "Not Found".as_bytes().to_vec())?;
            return Ok(());
        }

        let router_builder = router_builder.unwrap();
        let method = req.method();
        let handler = router_builder.lock().unwrap().get_handler(method);
        if handler.is_none() {
            Response::new(response_out).send_bytes(404, "Not Found".as_bytes().to_vec())?;
            return Ok(());
        }

        let handler = handler.unwrap();
        let path_map = { path_map.clone().lock().unwrap().clone() };
        let ctx = HttpContext::new(query_map, path_map, response_out, req);

        handler(ctx).context("handle request failed")?;

        Ok(())
    }
}
