use anyhow::Result;
use spin_sdk::pg::{self, ParameterValue, RowSet};

use db_adapter::DbAdapter;
use crate::models::Episode;

pub struct EpisodeDb {
    uri: String,
}

impl EpisodeDb {
    pub fn new(uri: String) -> Self {
        EpisodeDb { uri }
    }
}

impl DbAdapter<Episode, RowSet> for EpisodeDb {
    fn insert(&self, model: &Episode) -> Result<RowSet> {
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
        
        Ok(pg::query(&self.uri, sql, &params)?)
    }

    fn find_all(&self) -> Result<RowSet> {
        let sql = "SELECT * FROM episodes";
        
        Ok(pg::query(&self.uri, sql, &[])?)
    }

    fn find_one(&self, id: i32) -> Result<RowSet> {
        let sql = "SELECT * FROM episodes WHERE id=$1";
        let params = vec![ParameterValue::Int32(id)];
        
        Ok(pg::query(&self.uri, sql, &params)?)
    }

    fn update(&self, id: i32, model: &Episode) -> Result<RowSet> {
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
        Ok(pg::query(&self.uri, sql, &params)?)
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
