use crate::bindings::wasi::http::types::{IncomingRequest, Method};

pub struct Request {
    pub incoming_request: IncomingRequest,
}

impl Request {
    pub fn new(incoming_request: IncomingRequest) -> Self {
        Request { incoming_request }
    }

    pub fn path_with_query(&self) -> Option<String> {
        self.incoming_request.path_with_query()
    }

    pub fn method(&self) -> Method {
        self.incoming_request.method()
    }
}
