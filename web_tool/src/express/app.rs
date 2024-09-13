use crate::bindings::wasi::http::types::{IncomingRequest, ResponseOutparam};

use super::context;

pub struct App {
    pub handlers: Vec<Handler>,
}

impl App {
    pub fn new() -> Self {
        App {
            handlers: Vec::new(),
        }
    }

    pub fn layer(&mut self, handler: Handler) {
        self.handlers.push(handler)
    }

    pub fn handle(
        self,
        req: IncomingRequest,
        response_out: ResponseOutparam,
    ) -> anyhow::Result<()> {
        let mut ctx = context::Context::new(self, req, response_out);
        ctx.exec_handler()?;

        ctx.response.flush();
        Ok(())
    }
}

pub type Handler = fn(&mut super::context::Context) -> anyhow::Result<()>;
