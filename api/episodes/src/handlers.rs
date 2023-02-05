use anyhow::Result;
use spin_sdk::http::Response;

use rest_api::handlers::{bad_request, internal_server_error, not_found, ok};
use db_adapter::DbAdapter;

use crate::models::Episode;

/// Creates a new Episode
pub(crate) fn create<Db: DbAdapter<Episode>>(db: Db, episode: Episode) -> Result<Response> {
    match db.insert(&episode)? {
        Some(episode) => ok(serde_json::to_string(&episode)?),
        None => not_found(),
    }
}

/// Finds all Episodes
pub(crate) fn find_all<Db: DbAdapter<Episode>>(db: Db) -> Result<Response> {
    let episodes = db.find_all()?;

    ok(serde_json::to_string(&episodes)?)
}

/// Finds one record
pub(crate) fn find_by_id<Db: DbAdapter<Episode>>(db: Db, id: i32) -> Result<Response> {
    match db.find_one(id)? {
        Some(episode) => ok(serde_json::to_string(&episode)?),
        None => not_found(),
    }
}

/// Updates one record by ID
pub(crate) fn update<Db: DbAdapter<Episode>>(db: Db, id: i32, model: Episode) -> Result<Response> {
    let episode = db.update(id, &model)?;

    match episode {
        Some(episode) => ok(serde_json::to_string(&episode)?),
        None => not_found(),
    }
}

/// Deletes the primary record
pub(crate) fn delete<Db: DbAdapter<Episode>>(db: Db, id: i32) -> Result<Response> {
    match db.delete(id)? {
        1 => ok("success".into()), // TODO update
        0 => bad_request(),
        _ => internal_server_error(),
    }
}
