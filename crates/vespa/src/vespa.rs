use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::{
    adapter::SearchAdapter,
    fetch::{self, from_body},
    models::{VespaDocument, VespaResponse},
    query::SearchQuery,
};

pub struct Vespa {
    api_base: String,
}

fn format_document_uri(api_base: &str, path: &str) -> String {
    format!("{}/document/v1/{}", api_base, path)
}

/// The Vespa adapter, which impliments both the CRUD and
/// search methods
impl SearchAdapter for Vespa {
    /// Create a new Vespa adapter with the give base URL
    fn new(api_base: &str) -> Self {
        Self {
            api_base: api_base.to_string(),
        }
    }

    /// Wraps the Vespa `query` API with a single parameter, the
    /// Vespa SearchQuery struct.
    fn query<Model: for<'a> Deserialize<'a>>(
        &self,
        search_query: &SearchQuery,
    ) -> Result<Option<VespaDocument<Model>>> {
        let uri = format!("{}/search/", &self.api_base);
        let res = fetch::post(&uri, search_query)?;

        from_body(&res)
    }

    fn get<Model: for<'a> Deserialize<'a>>(
        &self,
        path: &str,
    ) -> Result<Option<VespaResponse<Model>>> {
        let uri = format_document_uri(&self.api_base, path);
        let res = fetch::get(&uri)?;

        from_body(&res)
    }

    fn create<Model: Serialize + for<'a> Deserialize<'a>>(
        &self,
        path: &str,
        payload: &Model,
    ) -> Result<Option<VespaResponse<Model>>> {
        let uri = format_document_uri(&self.api_base, path);
        let res = fetch::post(&uri, payload)?;

        from_body(&res)
    }

    fn update<Model: Serialize + for<'a> Deserialize<'a>>(
        &self,
        path: &str,
        payload: &Model,
    ) -> Result<Option<VespaResponse<Model>>> {
        let uri = format_document_uri(&self.api_base, path);
        let res = fetch::put(&uri, payload)?;

        from_body(&res)
    }

    fn remove<Model: for<'a> Deserialize<'a>>(
        &self,
        path: &str,
    ) -> Result<Option<VespaResponse<Model>>> {
        let uri = format_document_uri(&self.api_base, path);
        let res = fetch::delete(&uri)?;

        from_body(&res)
    }
}
