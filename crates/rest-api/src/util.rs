use anyhow::Result;
use bytes::Bytes;
use http::HeaderValue;
use serde::de::DeserializeOwned;
use spin_sdk::http::Request;

/// Converts the Request body into Bytes, then into a JSON object
pub(crate) fn from_bytes<Model: DeserializeOwned + 'static>(req: &Request) -> Result<Model> {
    let bytes: Bytes = req.body().clone().unwrap_or_default();
    let model: Model = serde_json::from_slice(&bytes)?;
    Ok(model)
}

/// Gets the ID from the path, if present
pub(crate) fn get_id_from_path(header_value: &HeaderValue) -> Result<Option<i32>> {
    match header_value.to_str()?.split('/').last() {
        Some(str) => match str.parse::<i32>() {
            Ok(value) => Ok(Some(value)),
            _ => Ok(None),
        },
        _ => Ok(None),
    }
}
