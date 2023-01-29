use anyhow::Result;
use spin_sdk::pg::{self, ParameterValue};

use crate::models::{as_episode, as_episodes, Episode};
use db_adapter::DbAdapter;

pub struct EpisodeDb {
    uri: String,
}

impl EpisodeDb {
    pub fn new(uri: String) -> Self {
        EpisodeDb { uri }
    }
}

impl DbAdapter<Episode> for EpisodeDb {
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

    fn find_all(&self) -> Result<Vec<Episode>> {
        let sql = "SELECT * FROM episodes";
        let rowset = pg::query(&self.uri, sql, &[])?;

        Ok(as_episodes(&rowset)?)
    }

    fn find_one(&self, id: i32) -> Result<Option<Episode>> {
        let sql = "SELECT * FROM episodes WHERE id=$1";
        let params = vec![ParameterValue::Int32(id)];
        let rowset = pg::query(&self.uri, sql, &params)?;

        Ok(match rowset.rows.first() {
            Some(row) => Some(as_episode(row)?),
            None => None,
        })
    }

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

    fn delete(&self, id: i32) -> Result<u64> {
        let sql = "
            DELETE FROM 
                episodes 
            WHERE id=$1
        ";
        let params = [ParameterValue::Int32(id)];

        Ok(pg::execute(&self.uri, sql, &params)?)
    }
}
