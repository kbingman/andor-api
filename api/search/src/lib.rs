use anyhow::Result;
use spin_sdk::{
    http::{internal_server_error, not_found, Request, Response},
    http_component,
};

use response::{as_empty_response, as_response};
use rest_api::handlers::ok;
use router::{router, Router};
use store::EpisodeStore;
use vespa::{adapter::SearchAdapter, vespa::Vespa};

use crate::models::Episode;

mod models;
mod query;
mod response;
mod router;
mod store;

/// Search handler
/// base search handler
fn search<Db: SearchAdapter>(store: EpisodeStore<Db>, query: Option<String>) -> Result<Response> {
    // Get the response from the Vespa Document
    let res = match store.query::<Episode>(query, 0, 30)? {
        Some(doc) => as_response(doc),
        None => as_empty_response(),
    };
    ok(serde_json::to_string(&res)?)
}

/// Series handler
/// shows a list of all episodes from the given series
fn get_series_episodes<Db: SearchAdapter>(
    store: EpisodeStore<Db>,
    slug: String,
) -> Result<Response> {
    let res = match store.find_all_episodes::<Episode>(slug, 0, 30)? {
        Some(doc) => as_response(doc),
        None => as_empty_response(),
    };
    ok(serde_json::to_string(&res)?)
}

/// A simple Spin / Vespa search endpoint.
#[http_component]
fn vespa_api(req: Request) -> Result<Response> {
    let uri = "http://localhost:8080";
    let db = Vespa::new(uri);
    let store = EpisodeStore::new(db);

    match router(&req) {
        Ok(route) => match route {
            Router::Search(query) => search(store, query),
            Router::FindAllEpisodes(slug) => get_series_episodes(store, slug),
            Router::FindEpisodeById(_slug, _id) => not_found(),
            Router::InternalServerError => internal_server_error(),
            Router::NotFound => not_found(),
        },
        _ => internal_server_error(),
    }
}
