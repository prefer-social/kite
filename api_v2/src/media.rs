use anyhow::{bail, Result};
use mime::Mime;
use spin_sdk::http::{Method, Params, Request, Response};
use std::collections::HashMap;
use std::str;

use crate::media::get::get;
use crate::media::post::post;

mod get;
mod post;

// https://docs.joinmastodon.org/methods/media/#v2
// Returns: MediaAttachment, but without a URL
pub async fn request(req: Request, params: Params) -> Result<Response> {
    match req.method() {
        Method::Post => post(req, params).await,
        Method::Get => get(req, params).await,
        _ => return crate::http_responses::notfound().await,
    }
}

pub fn get_multipart_boundary(req: &Request) -> Result<String> {
    let boundary = req.header("content-type").unwrap();
    let a = boundary.as_str().unwrap();
    if &a[..30] == "multipart/form-data; boundary=" {
        let mut split = a.split("boundary=").collect::<Vec<&str>>();
        let a = split[1];
        return Ok(a.to_string());
    }
    bail!("Can't find boundary from header")
}

pub fn allowed_mime_type(mime: &Mime) -> Result<Option<&str>> {
    let allowed_mime_and_extension = HashMap::from([
        (mime::IMAGE_GIF, "gif"),
        (mime::IMAGE_PNG, "png"),
        (mime::IMAGE_JPEG, "jpg"),
    ]);

    if let Some(ext) = allowed_mime_and_extension.get(&mime) {
        return Ok(Some(ext));
    }

    Ok(None)
}
