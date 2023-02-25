use crate::{document::VespaDocument, fetch, query::SearchQuery};
use anyhow::Result;
use serde::Deserialize;

/// A generic Adapter for the Vespa search endpoint
/// returns a Vespa document
pub trait SearchAdapter {
    fn new(uri: String) -> Self;
    
    fn query<Model: for<'a> Deserialize<'a>>(
        &self,
        query: &SearchQuery,
    ) -> Result<Option<VespaDocument<Model>>>;
}

pub struct Vespa {
    uri: String,
}

/// The Vespa adapter, which impliments the `query` method
impl SearchAdapter for Vespa {
    fn new(uri: String) -> Self {
        Self { uri }
    }
    
    fn query<Model: for<'a> Deserialize<'a>>(
        &self,
        search_query: &SearchQuery,
    ) -> Result<Option<VespaDocument<Model>>> {
        let uri = format!("{}/search/", &self.uri);
        let res = fetch::post(&uri, search_query)?;

        Ok(match res.body() {
            Some(body) => {
                let doc: VespaDocument<Model> = serde_json::from_slice(body)?;

                Some(doc)
            }
            None => None,
        })
    }
}
