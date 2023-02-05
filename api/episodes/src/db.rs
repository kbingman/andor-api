use anyhow::{Result, Context};
use spin_sdk::pg::{self, ParameterValue};
use db_adapter::DbAdapter;

use crate::models::{as_episode, aggregate_episodes, Episode};

pub struct EpisodeDb {
    uri: String,
}

impl EpisodeDb {
    pub fn new(uri: String) -> Self {
        EpisodeDb { uri }
    }
}

impl DbAdapter<Episode> for EpisodeDb {
    /// Insert 
    fn insert(&self, model: &Episode) -> Result<Option<Episode>> {
        let sql = "
            INSERT INTO 
                episodes (title, description, episode) 
            VALUES ($1, $2, $3) 
            RETURNING id, title, description, episode
        ";
        let params = vec![
            ParameterValue::Str(&model.title),
            ParameterValue::Str(&model.description),
            ParameterValue::Int32(model.episode),
        ];
        let rowset = pg::query(&self.uri, sql, &params)?;

        Ok(match rowset.rows.first() {
            Some(row) => Some(as_episode(row)?),
            None => None,
        })
    }

    /// Find All 
    fn find_all(&self) -> Result<Vec<Episode>> {
        let sql = "
            SELECT 
                episodes.id, 
                episodes.title, 
                episodes.description,
                episodes.episode,
                people_episodes.person_id
            FROM episodes
            LEFT JOIN people_episodes on (episodes.id = people_episodes.episode_id)
        ";
        let rowset = pg::query(&self.uri, sql, &[])?;

        Ok(aggregate_episodes(rowset)?)
    }

    /// Find One 
    fn find_one(&self, id: i32) -> Result<Option<Episode>> {
        let sql = "
            SELECT 
                episodes.id, 
                episodes.title, 
                episodes.description,
                episodes.episode,
                people_episodes.person_id
            FROM episodes
            LEFT JOIN people_episodes on (episodes.id = people_episodes.episode_id)
            WHERE id=$1
        ";
        let params = vec![ParameterValue::Int32(id)];
        let rowset = pg::query(&self.uri, sql, &params)?;
        let results = aggregate_episodes(rowset)?;

        Ok(match results.first() {
            Some(episode) => Some(episode.to_owned()),
            _ => None,
        })
    }

    /// Update 
    fn update(&self, id: i32, model: &Episode) -> Result<Option<Episode>> {
        let sql = "
            UPDATE 
                episodes 
            SET 
                title=$2 description=$3 
            WHERE id=$1 
            RETURNING id, title, description
        ";
        let params = vec![
            ParameterValue::Int32(id),
            ParameterValue::Str(&model.title),
            ParameterValue::Str(&model.description),
        ];
        let rowset = pg::query(&self.uri, sql, &params)?;

        Ok(match rowset.rows.first() {
            Some(row) => Some(as_episode(row)?),
            None => None,
        })
    }

    /// Deletes the primary record and the associated join
    /// table rows. 
    fn delete(&self, id: i32) -> Result<u64> {        
        let result = pg::execute(
            &self.uri,
            "DELETE FROM episodes WHERE id=$1",
            &[ParameterValue::Int32(id)],
        ).context("Error removing episodes.")?;
        
        pg::execute(
            &self.uri,
            "DELETE FROM people_episodes WHERE episode_id=$1",
            &[ParameterValue::Int32(id)],
        ).context("Error removing join table data for episodes and people")?;

        Ok(result)
    }
}
