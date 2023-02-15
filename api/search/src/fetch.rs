use anyhow::Result;
use bytes::Bytes;
use spin_sdk::{http::Response, outbound_http::send_request};

/// A simple wrapper around the Spin `send_request` module
/// with a few application specific defaults. 
/// The `application/json` content-type is hardwired into 
/// this, which is required for Vespa requests
pub fn fetch(uri: &str, method: http::Method, body: Option<Bytes>) -> Result<Response> {
    let res = send_request(
        http::Request::builder()
            .method(method)
            .header(http::header::CONTENT_TYPE, "application/json")
            .uri(uri)
            .body(body)?,
    )?;

    Ok(res)
}
