use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_querystring::{from_str, ParseMode};
use spin_sdk::http::Request;

#[derive(Debug, Deserialize, Serialize)]
struct QueryString {
    q: Option<String>,
}

/// Gets the `q` query parameter from the Request object
/// If `q` is not present, returns an empty string
pub fn get_query(req: &Request) -> Result<Option<String>> {
    Ok(match req.uri().query() {
        Some(qs) => {
            let res: QueryString = from_str(qs, ParseMode::UrlEncoded)?;
            res.q
        }
        None => None,
    })
}
