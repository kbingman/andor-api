use std::collections::HashMap;

use anyhow::Result;
use serde::Deserialize;
use vespa::{
    document::VespaDocument,
    query::{Presentation, SearchQuery},
    search::SearchAdapter,
};

pub struct EpisodeDb<Db: SearchAdapter> {
    db: Db,
}

impl<Db: SearchAdapter> EpisodeDb<Db> {
    pub fn new(uri: &str) -> Self {
        let db = Db::new(uri.to_string());
        Self { db }
    }

    /// The actual query for finding Episodes, takes
    /// a query, offset, and size arguments
    pub fn query<T: for<'a> Deserialize<'a>>(
        &self,
        query: Option<String>,
        offset: u32,
        hits: u32,
    ) -> Result<Option<VespaDocument<T>>> {
        // The YQL query
        let yql = "
            select * from episodes where {targetHits: 100}nearestNeighbor(embedding, e) 
            AND 
            userQuery()
        "
        .to_string();
    
        // The input argument
        let mut input = HashMap::new();
        match &query {
            Some(q) => {
                input.insert("query(e)".to_string(), format!("embed({})", q));
            }
            None => {}
        };
    
        // And the complete SearchQuery
        let search_query = SearchQuery {
            yql,
            query,
            input,
            hits,
            offset,
            query_type: "weakAnd".to_string(),
            presentation: Presentation {
                bolding: true,
                format: "json".to_string(),
            },
        };
        
        let res = self.db.query(&search_query)?;

        Ok(res)
    }
}
