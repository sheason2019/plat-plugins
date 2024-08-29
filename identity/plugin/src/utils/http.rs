use std::{fs, path::Path};

use crate::bindings::wasi::http::types::{
    Fields, OutgoingBody, OutgoingResponse, ResponseOutparam,
};

pub fn send_file(uri_path: String, response_out: ResponseOutparam) -> anyhow::Result<()> {
    let mut uri_path = uri_path.clone();
    uri_path.remove(0);

    let file_path = Path::new("/storage").join(uri_path);
    if !file_path.exists() {
        let resp = OutgoingResponse::new(Fields::new());
        resp.set_status_code(404).expect("set status code");
        let body = resp.body().expect("outgoing response");

        ResponseOutparam::set(response_out, Ok(resp));
        body.write()
            .expect("outgoing stream")
            .blocking_write_and_flush(&"Not Found".as_bytes())?;

        OutgoingBody::finish(body, None).expect("finish not found request");
    } else {
        let file_bytes = fs::read(&file_path)?;

        let guess = mime_guess::from_path(&file_path);
        let mime = guess.first_raw().unwrap();

        let headers = Fields::new();
        headers.append(&"Content-Type".to_string(), &mime.as_bytes().to_vec())?;

        let resp = OutgoingResponse::new(headers);
        let body = resp.body().expect("outgoing response");

        ResponseOutparam::set(response_out, Ok(resp));

        body.write()
            .expect("outgoing stream")
            .blocking_write_and_flush(&file_bytes)?;

        OutgoingBody::finish(body, None).expect("finish request");
    }

    Ok(())
}
