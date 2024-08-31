use std::{fs, path::PathBuf};

use crate::bindings::wasi::http::types::{
    Fields, OutgoingBody, OutgoingResponse, ResponseOutparam, StatusCode,
};

pub struct Response {
    headers: Fields,
    status_code: StatusCode,
    body_bytes: Vec<u8>,
    response_out: ResponseOutparam,
}

impl Response {
    pub fn new(response_out: ResponseOutparam) -> Self {
        Response {
            headers: Fields::new(),
            status_code: 404,
            body_bytes: Vec::new(),
            response_out,
        }
    }

    pub fn status(&mut self, status: StatusCode) {
        self.status_code = status;
    }

    pub fn send_bytes(
        mut self,
        status_code: StatusCode,
        body_bytes: Vec<u8>,
    ) -> anyhow::Result<()> {
        self.status(status_code);
        self.body_bytes = body_bytes;
        self.flush()
    }

    pub fn send_file(mut self, file_path: PathBuf) -> anyhow::Result<()> {
        if !file_path.exists() {
            self.status(404);
            return Ok(());
        }

        self.status(200);
        self.body_bytes = fs::read(&file_path)?;

        let guess = mime_guess::from_path(&file_path);
        let mime = guess.first_raw().unwrap();

        self.headers
            .append(&"Content-Type".to_string(), &mime.as_bytes().to_vec())?;

        self.flush()
    }

    pub fn send_json(self, status_code: StatusCode, json_string: String) -> anyhow::Result<()> {
        self.headers.append(
            &"Content-Type".to_string(),
            &"application/json".as_bytes().to_vec(),
        )?;
        self.send_bytes(status_code, json_string.into_bytes())
    }

    pub fn flush(self) -> anyhow::Result<()> {
        let resp = OutgoingResponse::new(self.headers.clone());
        resp.set_status_code(self.status_code)
            .expect("set status code");
        let body = resp.body().expect("outgoing response");

        ResponseOutparam::set(self.response_out, Ok(resp));

        let body_len = self.body_bytes.len();
        let chunk_size = 2048;
        let mut chunk_count = body_len / 2048;
        if body_len % 2048 != 0 {
            chunk_count = chunk_count + 1;
        }

        let out = body.write().expect("outgoing stream");
        for i in 0..chunk_count {
            let mut end = (i + 1) * chunk_size;
            if end > body_len {
                end = body_len;
            }
            let chunk = &self.body_bytes[i * chunk_size..end];
            out.blocking_write_and_flush(chunk)?;
        }
        drop(out);

        OutgoingBody::finish(body, None).expect("finish request");

        Ok(())
    }
}
