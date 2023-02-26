use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::{
    models::{VespaDocument, VespaResponse},
    query::SearchQuery,
};

/// A generic Adapter for the Vespa search endpoint
/// returns a Vespa document
pub trait SearchAdapter {
    fn new(api_base: &str) -> Self;

    fn query<Model: for<'a> Deserialize<'a>>(
        &self,
        query: &SearchQuery,
    ) -> Result<Option<VespaDocument<Model>>>;

    fn get<Model: for<'a> Deserialize<'a>>(
        &self,
        api_base: &str,
    ) -> Result<Option<VespaResponse<Model>>>;

    fn create<Model: Serialize + for<'a> Deserialize<'a>>(
        &self,
        api_base: &str,
        payload: &Model,
    ) -> Result<Option<VespaResponse<Model>>>;

    fn update<Model: Serialize + for<'a> Deserialize<'a>>(
        &self,
        api_base: &str,
        payload: &Model,
    ) -> Result<Option<VespaResponse<Model>>>;

    fn remove<Model: for<'a> Deserialize<'a>>(
        &self,
        api_base: &str,
    ) -> Result<Option<VespaResponse<Model>>>;
}
