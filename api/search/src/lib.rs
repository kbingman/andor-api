use anyhow::Result;
use spin_sdk::{
    http::{Request, Response},
    http_component,
};

use db::{SearchAdapter, VespaDb};
use query::get_query;
use response::{as_empty_response, as_response};
use rest_api::handlers::ok;

mod db;
mod episode;
mod fetch;
mod query;
mod response;
mod vespa;

/// A simple Spin / Vespa search endpoint.
#[http_component]
fn vespa_api(req: Request) -> Result<Response> {
    let uri = "http://localhost:8080";
    let db = VespaDb::new(uri.to_string());
    let query = match get_query(&req)? {
        Some(q) => q,
        _ => "".to_string()
    };
    let doc = db.search(&query)?;

    let res = match doc {
        Some(doc) => as_response(doc),
        None => as_empty_response(),
    };

    ok(serde_json::to_string(&res)?)
}
