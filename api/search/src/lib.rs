use anyhow::Result;
use spin_sdk::{
    http::{Request, Response},
    http_component,
};

use db::EpisodeDb;
use query::get_query;
use response::{as_empty_response, as_response};
use rest_api::handlers::ok;
use vespa::{adapter::SearchAdapter, vespa::Vespa};

use crate::episode::Episode;

mod db;
mod episode;
mod query;
mod response;

/// A simple Spin / Vespa search endpoint.
#[http_component]
fn vespa_api(req: Request) -> Result<Response> {
    let uri = "http://localhost:8080";
    let db = Vespa::new(uri);
    let store = EpisodeDb::new(db);

    let query = get_query(&req)?;

    // Get the response from the Vespa Document
    let res = match store.query::<Episode>(query, 0, 20)? {
        Some(doc) => as_response(doc),
        None => as_empty_response(),
    };

    ok(serde_json::to_string(&res)?)
}
