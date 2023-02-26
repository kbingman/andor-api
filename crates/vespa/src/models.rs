use serde::{Deserialize, Serialize};

/// The children, if any, of the Vespa Document
#[derive(Clone, Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct VespaChildren<T> {
    pub fields: T,
    id: String,
    relevance: f64,
    source: String,
}

/// The Vespa Document Fields
#[derive(Clone, Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct VespaFields {
    pub total_count: i32,
}

/// The Vespa Document Root
#[derive(Clone, Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct VespaDocRoot<T> {
    id: String,
    pub relevance: f64,
    pub fields: VespaFields,
    pub children: Option<Vec<VespaChildren<T>>>,
}

/// The base Vespa Document, wraps the `root` element
#[derive(Clone, Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct VespaDocument<T> {
    pub root: VespaDocRoot<T>,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct VespaResponse<T> {
    pub path_id: String,
    pub id: String,
    fields: T,
}
