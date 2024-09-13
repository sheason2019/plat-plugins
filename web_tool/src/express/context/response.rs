use std::{
    fs::File,
    io::{BufReader, Read},
    path::PathBuf,
};

use crate::bindings::wasi::http::types::{Fields, OutgoingBody, OutgoingResponse, ResponseOutparam, StatusCode};

enum BodyType {
    None,
    File,
    Text,
}

pub struct Response {
    response_outparam: ResponseOutparam,

    body: Body,
    pub status: StatusCode,
    pub headers: Fields,
}

struct Body {
    ty: BodyType,
    text: Option<String>,
    path: Option<PathBuf>,
}

impl Response {
    pub fn new(response_outparam: ResponseOutparam) -> Self {
        Response {
            response_outparam,
            body: Body {
                ty: BodyType::None,
                text: None,
                path: None,
            },
            status: 0,
            headers: Fields::new(),
        }
    }

    pub fn text_body(&mut self, text: String) {
        self.body = Body {
            ty: BodyType::Text,
            text: Some(text),
            path: None,
        };
    }

    pub fn file_body(&mut self, path: PathBuf) {
        self.body = Body {
            ty: BodyType::File,
            text: None,
            path: Some(path),
        };
    }

    pub fn flush(self) {
        let response = OutgoingResponse::new(self.headers.clone());
        response.set_status_code(self.status).unwrap();

        let body_text = match self.body.text {
            Some(value) => value,
            None => "".to_string(),
        };
        let body_bytes = body_text.as_bytes();

        let mut reader: Box<dyn Read> = match self.body.ty {
            BodyType::None => return,
            BodyType::File => {
                let input = File::open(self.body.path.unwrap()).unwrap();
                Box::new(BufReader::new(input))
            }
            BodyType::Text => Box::new(body_bytes),
        };

        let body = response.body().unwrap();
        ResponseOutparam::set(self.response_outparam, Ok(response));

        let out = body.write().unwrap();
        let mut buf: [u8; 1024] = [0; 1024];
        while let Ok(nbytes_read) = reader.read(&mut buf) {
            if nbytes_read == 0 {
                break;
            }

            out.blocking_write_and_flush(&buf[0..nbytes_read]).unwrap();
        }
        drop(out);

        OutgoingBody::finish(body, None).unwrap();
    }
}
