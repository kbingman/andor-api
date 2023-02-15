use anyhow::Result;
use vespa::document::VespaDocument;

use crate::{
    episode::Episode, 
    vespa::{SearchQuery, Presentation}, fetch::fetch
};

/// A generic Adapter for the Vespa search endpoint
/// returns a Vespa document
pub trait SearchAdapter<Model> {
    fn search(&self, query: &str) -> Result<Option<VespaDocument<Model>>>;
}

pub struct VespaDb {
    uri: String,
}

impl VespaDb {
    pub fn new(uri: String) -> Self {
        VespaDb { uri }
    }
}

/// A simple function to format Vespa search queries
/// This is directly tied to how the search model works
fn format_search_query(query: String, offset: u32) -> SearchQuery {
    SearchQuery {
        yql: "select * from episodes where userQuery()".to_string(),
        query,
        offset,
        query_type: "weakAnd".to_string(),
        presentation: Presentation {
            bolding: true,
            format: "json".to_string(),
        },
    }
}

impl SearchAdapter<Episode> for VespaDb {
    fn search(&self, query: &str) -> Result<Option<VespaDocument<Episode>>> {
        let uri = format!("{}/search/", &self.uri);
        let offset = 0;
        let search_query = format_search_query(query.to_string(), offset);

        let res = fetch(
            &uri,
            http::Method::POST,
            Some(serde_json::to_vec(&search_query)?.into()),
        )?;

        Ok(match res.body() {
            Some(body) => {
                let doc: VespaDocument<Episode> = serde_json::from_slice(body)?;

                Some(doc)
            }
            None => None,
        })
    }
}