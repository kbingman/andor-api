use anyhow::Result;
use bytes::Bytes;
use serde::{Deserialize, Serialize};
use spin_sdk::{http::Response, outbound_http::send_request};

/// Converts a serializable struct into bytes, used for making
pub(crate) fn from_body<T: for<'a> Deserialize<'a>>(res: &Response) -> Result<Option<T>> {
    Ok(match res.body() {
        Some(body) => {
            let doc: T = serde_json::from_slice(body)?;

            Some(doc)
        }
        None => None,
    })
}

/// Converts a serializable struct into bytes, used for making
/// http requests
pub(crate) fn as_bytes<T: Serialize>(payload: &T) -> Result<Bytes> {
    Ok(serde_json::to_vec(payload)?.into())
}

/// A simple wrapper around the Spin `send_request` module
/// with a few application specific defaults.
/// The `application/json` content-type is hardwired into
/// this, which is required for Vespa requests
pub(crate) fn fetch(uri: &str, method: http::Method, body: Option<Bytes>) -> Result<Response> {
    let res = send_request(
        http::Request::builder()
            .method(method)
            .header(http::header::CONTENT_TYPE, "application/json")
            .uri(uri)
            .body(body)?,
    )?;

    Ok(res)
}

/// A wrapper around a GET request using the `fetch` method
pub(crate) fn get(uri: &str) -> Result<Response> {
    fetch(uri, http::Method::GET, None)
}

/// A wrapper around a POST request using the `fetch` method
pub(crate) fn post<T: Serialize>(uri: &str, payload: &T) -> Result<Response> {
    fetch(uri, http::Method::POST, Some(as_bytes(payload)?))
}

/// A wrapper around a PUT request using the `fetch` method
pub(crate) fn put<T: Serialize>(uri: &str, payload: &T) -> Result<Response> {
    fetch(uri, http::Method::PUT, Some(as_bytes(payload)?))
}

/// A wrapper around a DELETE request using the `fetch` method
pub(crate) fn delete(uri: &str) -> Result<Response> {
    fetch(uri, http::Method::DELETE, None)
}
