use anyhow::Result;
use bytes::Bytes;
use serde::{Serialize, Deserialize};
use spin_sdk::{http::Response, outbound_http::send_request};

/// Converts a serializable struct into bytes, used for making 
/// http requests
pub fn from_bytes<T: for<'a> Deserialize<'a>>(body: &Bytes) -> Result<T> {
    Ok(serde_json::from_slice(body)?)
}

/// Converts a serializable struct into bytes, used for making 
/// http requests
pub fn as_bytes<T: Serialize>(payload: &T) -> Result<Bytes> {
    Ok(serde_json::to_vec(payload)?.into())
}

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

pub fn post<T: Serialize>(uri: &str, payload: &T) -> Result<Response> {
    fetch(uri, http::Method::POST, Some(as_bytes(payload)?))
}
