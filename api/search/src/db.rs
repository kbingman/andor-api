use anyhow::Result;
use vespa::document::VespaDocument;

use crate::{
    episode::Episode,
    fetch::{self, fetch},
    vespa::{Presentation, SearchQuery},
};

/// A generic Adapter for the Vespa search endpoint
/// returns a Vespa document
pub trait SearchAdapter<Model> {
    fn query(&self, query: &str) -> Result<Option<VespaDocument<Model>>>;
}

pub struct VespaDb {
    uri: String,
}

impl VespaDb {
    pub fn new(uri: String) -> Self {
        VespaDb { uri }
    }
}

impl SearchAdapter<Episode> for VespaDb {
    // Performs a search using 
    fn query(&self, query: &str) -> Result<Option<VespaDocument<Episode>>> {
        let uri = format!("{}/search/", &self.uri);
        let search_query = SearchQuery {
            yql: "select * from episodes where {targetHits: 100}nearestNeighbor(embedding, e) AND userQuery()".to_string(),
            query: Some(query.to_string()),
            input: Some(format!("embed({})", query)),
            hits: 30,
            offset: 0,
            query_type: "weakAnd".to_string(),
            presentation: Presentation {
                bolding: true,
                format: "json".to_string(),
            },
        };

        let res = fetch(
            &uri,
            http::Method::POST,
            Some(serde_json::to_vec(&search_query)?.into()),
        )?;
        
        // let res = fetch::post(&uri, &search_query)?;

        Ok(match res.body() {
            Some(body) => {
                let doc: VespaDocument<Episode> = serde_json::from_slice(body)?;

                Some(doc)
            }
            None => None,
        })
    }
}
