use std::collections::HashMap;

use anyhow::Result;
use serde::{Deserialize, Serialize};
use spin_sdk::pg::{Decode, Row, RowSet};

// A trait generic over `T`.
pub trait FromRow {
    fn from_row(row: &Row) -> Result<Episode>;
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Episode {
    pub id: Option<i32>,
    pub title: String,
    pub description: String,
    pub episode: i32,
    pub people_ids: Vec<i32>,
}

pub fn as_episode(row: &Row) -> Result<Episode> {
    let id = i32::decode(&row[0])?;
    let title = String::decode(&row[1])?;
    let description = String::decode(&row[2])?;
    let episode = i32::decode(&row[3])?;
    
    let people_ids: Vec<i32> = if row.len() >= 5 {
        match i32::decode(&row[4]) {
            Ok(id) => vec![id],
            _ => Vec::new(),
        }
    } else {
        Vec::new()
    };

    Ok(Episode {
        id: Some(id),
        title,
        description,
        episode,
        people_ids,
    })
}

/// The current DB query returns a row for each left join. Because `spin::pg` 
/// has limited support for datatypes, this takes each row and aggregrates it
/// by ID. The episode_ids are then pushed into an array. 
pub(crate) fn aggregate_episodes(rowset: RowSet) -> Result<Vec<Episode>> {
    let hashmap: Result<HashMap<i32, Episode>> =
        rowset
            .rows
            .iter()
            .try_fold(HashMap::<i32, Episode>::new(), |mut acc, row| {
                let mut episode = as_episode(row)?;
                match episode.id {
                    Some(id) => match acc.get(&id) {
                        Some(record) => {
                            episode.people_ids.append(&mut record.people_ids.clone());
                            episode.people_ids.sort();
                            acc.insert(id, episode);
                        }
                        None => {
                            acc.insert(id, episode);
                        }
                    },
                    _ => {}
                }

                Ok(acc)
            });

    let mut episodes: Vec<Episode> = hashmap?.values().cloned().collect();
    episodes.sort_by(|a, b| a.episode.cmp(&b.episode));

    Ok(episodes)
}
