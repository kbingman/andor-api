use serde::{Deserialize, Serialize};
use vespa::models::VespaDocument;

#[derive(Clone, Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SearchResponse<T> {
    pub total_count: i32,
    pub results: Vec<T>,
}

/// Returns an empty response matching the vespa search response below
pub fn as_empty_response<T: Clone>() -> SearchResponse<T> {
    SearchResponse {
        total_count: 0,
        results: Vec::new(),
    }
}

/// Returns a formatted vespa response with total count and results
pub fn as_response<T: Clone>(doc: VespaDocument<T>) -> SearchResponse<T> {
    let total_count = doc.root.fields.total_count;
    let results: Vec<T> = match doc.root.children {
        Some(children) => children.iter().map(|c| c.fields.to_owned()).collect(),
        None => Vec::new(),
    };

    SearchResponse {
        total_count,
        results,
    }
}
