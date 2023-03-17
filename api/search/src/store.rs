use std::collections::HashMap;

use anyhow::Result;
use serde::Deserialize;
use vespa::{
    adapter::SearchAdapter,
    models::VespaDocument,
    query::{Presentation, SearchQuery},
};

pub struct EpisodeStore<Db: SearchAdapter> {
    db: Db,
}

/// The Vespa DB Episode search
///
impl<Db: SearchAdapter> EpisodeStore<Db> {
    pub fn new(db: Db) -> Self {
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

    /// Finds all episodes for a given series
    pub fn find_all_episodes<T: for<'a> Deserialize<'a>>(
        &self,
        series: String,
        offset: u32,
        hits: u32,
    ) -> Result<Option<VespaDocument<T>>> {
        let search_query = SearchQuery {
            yql: format!(
                "select * from episodes where series contains \"{}\"",
                series
            ),
            query: None,
            input: HashMap::new(),
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
