use anyhow::Result;
use rest_api::api::{get_api_from_request, Api};
use rest_api::handlers::{bad_request, internal_server_error, method_not_allowed, not_found, ok};
use spin_sdk::pg::RowSet;
use spin_sdk::{
    http::{Request, Response},
    http_component,
};

use db_adapter::DbAdapter;
use crate::db::EpisodeDb;
use crate::models::{as_episode, as_episodes, Episode};

mod db;
mod models;

/// Creates a new Episode
pub(crate) fn create<Db: DbAdapter<Episode, RowSet>>(db: Db, episode: Episode) -> Result<Response> {
    match db.insert(&episode) {
        Ok(rowset) => match rowset.rows.first() {
            Some(row) => ok(serde_json::to_string(&as_episode(row)?)?),
            None => not_found(),
        },
        Err(err) => {
            println!("Error: {:#?}", err);
            bad_request()
        }
    }
}

/// Finds all Episodes
pub(crate) fn find_all<Db: DbAdapter<Episode, RowSet>>(db: Db) -> Result<Response> {
    let rowset = db.find_all()?;

    ok(serde_json::to_string(&as_episodes(&rowset)?)?)
}

/// Finds one record
pub(crate) fn find_by_id<Db: DbAdapter<Episode, RowSet>>(db: Db, id: i32) -> Result<Response> {
    let rowset = db.find_one(id)?;

    match rowset.rows.first() {
        Some(row) => ok(serde_json::to_string(&as_episode(row)?)?),
        None => not_found(),
    }
}

/// Updates one record by ID
pub(crate) fn update<Db: DbAdapter<Episode, RowSet>>(db: Db, id: i32, model: Episode) -> Result<Response> {
    let rowset = db.update(id, &model)?;

    match rowset.rows.first() {
        Some(row) => ok(serde_json::to_string(&as_episode(row)?)?),
        None => not_found(),
    }
}

pub(crate) fn delete<Db: DbAdapter<Episode, RowSet>>(db: Db, id: i32) -> Result<Response> {
    match db.delete(id)? {
        1 => ok("success".into()), // TODO update
        0 => bad_request(),
        _ => internal_server_error(),
    }
}

#[http_component]
fn episode_api(req: Request) -> Result<Response> {
    let uri = spin_sdk::config::get("postgres_uri")?;
    let api: Api<Episode> = get_api_from_request(req)?;
    let db = EpisodeDb::new(uri);

    match api {
        Api::Create(model) => create(db, model),
        Api::FindAll => find_all(db),
        Api::FindById(id) => find_by_id(db, id),
        Api::Update(id, model) => update(db, id, model),
        Api::Delete(id) => delete(db, id),
        Api::MethodNotAllowed => method_not_allowed(),
        Api::NotFound => not_found(),
        Api::BadRequest => bad_request(),
        Api::InternalServerError => internal_server_error(),
    }
}
