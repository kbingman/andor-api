use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct VespaResult<T> {
    pub fields: T,
    id: String,
    relevance: f64,
    source: String,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct VespaFields {
    pub total_count: i32,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct VespaDocRoot<T> {
    id: String,
    pub relevance: f64,
    pub fields: VespaFields,
    pub children: Option<Vec<VespaResult<T>>>,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct VespaDocument<T> {
    pub root: VespaDocRoot<T>,
}
