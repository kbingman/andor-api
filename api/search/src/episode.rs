use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Episode {
    series: Option<String>,
    // episode_id: Option<u32>,
    title: String,
    description: String,
}
