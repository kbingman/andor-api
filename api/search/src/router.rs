use anyhow::Result;
use http::HeaderValue;
use spin_sdk::http::Request;

use crate::query::get_query;

pub enum Router {
    Search(Option<String>),
    FindAllEpisodes(String),
    FindEpisodeById(String, String),
    NotFound,
    InternalServerError,
}

/// Gets the ID from the path, if present
pub(crate) fn get_path_segments<'a>(
    path: &'a Option<&'a HeaderValue>,
) -> Result<Option<Vec<&'a str>>> {
    Ok(match path {
        Some(value) => Some(value.to_str()?.split('/').collect::<Vec<&str>>()),
        _ => None,
    })
}

/// Series routes
///
/// `/series` - not implemented
/// `/series/[slug]` - shows a list of episodes
/// `/series/[slug]/[id]` - shows a single episodes
///
fn get_series(segments: &[&str]) -> Router {
    match segments.get(2) {
        Some(slug) => match segments.get(3) {
            None => Router::FindAllEpisodes(slug.to_string()),
            Some(id) => Router::FindEpisodeById(slug.to_string(), id.to_string()),
        },
        // Temp, may add listing of series
        None => Router::NotFound,
    }
}

/// Base router
///
/// `/search` - base search
/// `/series` - not implemented (404)
/// `/series/[slug]` - shows a list of episodes
/// `/series/[slug]/[id]` - shows a single episodes
///
pub(crate) fn router(req: &Request) -> Result<Router> {
    let path = &req.headers().get("spin-path-info");

    Ok(match get_path_segments(path)? {
        Some(segments) => match segments.get(1) {
            Some(route) => match route.to_owned() {
                "search" => Router::Search(get_query(req)?),
                "series" => get_series(&segments),
                _ => Router::NotFound,
            },
            None => Router::NotFound,
        },
        None => Router::InternalServerError,
    })
}
