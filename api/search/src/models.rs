use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Episode {
    series: Option<String>,
    season: Option<i32>,
    #[serde(rename(deserialize = "episode_id", serialize = "episodeId"))]
    episode_id: Option<i32>,
    title: String,
    description: Option<String>,
}
