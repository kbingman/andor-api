use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct Presentation {
    pub bolding: bool,
    pub format: String,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct SearchQuery {
    pub yql: String,
    pub query: Option<String>,
    // #[serde(rename = "input.query(e)")]
    pub input: HashMap<String, String>,
    pub hits: u32,
    pub offset: u32,
    #[serde(rename = "type")]
    pub query_type: String,
    pub presentation: Presentation,
}
