use std::{borrow::BorrowMut, path::PathBuf, sync::Arc};

use serde::Serialize;

use crate::bindings::wasi::http::types::{IncomingRequest, ResponseOutparam, StatusCode};

use super::app::App;

pub mod request;
pub mod response;

pub struct Context {
    pub request: request::Request,
    pub response: response::Response,

    handler_ptr: usize,
    app: Arc<App>,
}

impl Context {
    pub fn new(app: App, req: IncomingRequest, response_out: ResponseOutparam) -> Self {
        Self {
            request: request::Request::new(req),
            response: response::Response::new(response_out),
            app: Arc::new(app),
            handler_ptr: 0,
        }
    }

    pub fn exec_handler(&mut self) -> anyhow::Result<()> {
        if self.app.handlers.len() > self.handler_ptr {
            self.app.handlers[self.handler_ptr](self.borrow_mut())?;
        }

        Ok(())
    }

    pub fn next(mut self) -> anyhow::Result<()> {
        self.handler_ptr = self.handler_ptr + 1;
        self.exec_handler()
    }

    pub fn json<T>(&mut self, status_code: StatusCode, value: &T)
    where
        T: ?Sized + Serialize,
    {
        let json_string = serde_json::to_string(value).unwrap();
        self.response.status = status_code;
        self.response.text_body(json_string);
    }

    pub fn text(&mut self, status_code: StatusCode, text: String) {
        self.response.status = status_code;
        self.response.text_body(text);
    }

    pub fn file(&mut self, file_path: PathBuf) {
        if file_path.is_dir() || !file_path.exists() {
            self.text(404, "Not Found".to_string());
            return;
        }

        let guess = mime_guess::from_path(&file_path);
        let mime = guess.first_raw().unwrap();

        self.response
            .headers
            .append(&"Content-Type".to_string(), &mime.as_bytes().to_vec())
            .unwrap();

        self.response.status = 200;
        self.response.file_body(file_path);
    }
}
