use anyhow::Result;
use serde::{Deserialize, Serialize};
use spin_sdk::pg::{Decode, Row};

// A trait generic over `T`.
pub trait FromRow {
    fn from_row(row: &Row) -> Result<Episode>;
}

#[derive(Deserialize, Serialize)]
pub(crate) struct RawEpisode {
    pub title: String,
    pub people_ids: Vec<i32>,
}

#[derive(Deserialize, Serialize)]
pub struct Episode {
    pub title: String,
    pub id: i32,
    // pub created_at: String;
}

pub fn as_episode(row: &Row) -> Result<Episode> {
    println!("row {:#?}", row);
    let id = i32::decode(&row[0])?;
    let title = String::decode(&row[1])?;

    println!("{:#?}", row);

    Ok(Episode { id, title })
}
