use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct Presentation {
    pub bolding: bool,
    pub format: String,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct SearchQuery {
    pub yql: String,
    pub query: String,
    pub offset: u32,
    #[serde(rename = "type")]
    pub query_type: String,
    pub presentation: Presentation,
}
