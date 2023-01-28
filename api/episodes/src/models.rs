use anyhow::Result;
use serde::{Deserialize, Serialize};
use spin_sdk::pg::{Decode, Row, RowSet};

// A trait generic over `T`.
pub trait FromRow {
    fn from_row(row: &Row) -> Result<Episode>;
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Episode {
    pub id: Option<i32>,
    pub title: String,
    pub description: String,
    pub episode: i32,
    // pub created_at: String;
}

pub fn as_episode(row: &Row) -> Result<Episode> {
    let id = i32::decode(&row[0])?;
    let title = String::decode(&row[1])?;
    let description = String::decode(&row[2])?;
    let episode = i32::decode(&row[3])?;

    Ok(Episode {
        id: Some(id),
        title,
        description,
        episode,
    })
}

pub fn as_episodes(rowset: &RowSet) -> Result<Vec<Episode>> {
    rowset.rows.iter().map(as_episode).collect()
}
