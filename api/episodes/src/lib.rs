use anyhow::Result;
use rest_api::api::{get_api_from_request, Api};
use rest_api::handlers::{bad_request, internal_server_error, method_not_allowed, not_found, ok};
use spin_sdk::{
    http::{Request, Response},
    http_component,
    pg::{self, ParameterValue},
};

use crate::models::{as_episode, Episode, RawEpisode};

mod models;

/// Creates one record
pub(crate) fn create(uri: &str, model: RawEpisode) -> Result<Response> {
    let sql = "INSERT INTO episodes (title) VALUES ($1) RETURNING id, title";
    let params = vec![ParameterValue::Str(&model.title)];

    let rowset = pg::query(uri, sql, &params)?;

    match rowset.rows.first() {
        Some(row) => ok(serde_json::to_string(&as_episode(row)?)?),
        None => not_found(),
    }
}

/// Finds all People in the DB
pub(crate) fn find_all(uri: &str) -> Result<Response> {
    let sql = "SELECT * FROM episodes";
    let rowset = pg::query(uri, sql, &[])?;
    let results: Result<Vec<Episode>> = rowset.rows.iter().map(as_episode).collect();

    ok(serde_json::to_string(&results?)?)
}

/// Finds one record by ID
pub(crate) fn find_one(uri: &str, id: i32) -> Result<Response> {
    let sql = "SELECT * FROM episodes WHERE id=$1";
    let params = vec![ParameterValue::Int32(id)];
    let rowset = pg::query(uri, sql, &params)?;

    match rowset.rows.first() {
        Some(row) => ok(serde_json::to_string(&as_episode(row)?)?),
        None => not_found(),
    }
}

/// Updates one record by ID
pub(crate) fn update(address: &str, id: i32, model: RawEpisode) -> Result<Response> {
    let sql = "UPDATE episodes SET title=$2 WHERE id=$1 RETURNING id, title";
    let params = vec![ParameterValue::Int32(id), ParameterValue::Str(&model.title)];
    let rowset = pg::query(address, sql, &params)?;

    match rowset.rows.first() {
        Some(row) => ok(serde_json::to_string(&as_episode(row)?)?),
        None => not_found(),
    }
}

pub(crate) fn delete(uri: &str, id: i32) -> Result<Response> {
    let params = [ParameterValue::Int32(id)];
    // let rowset = pg::query(uri, "SELECT * FROM episodes WHERE id=$1", &params)?;

    match pg::execute(uri, "DELETE FROM episodes WHERE id=$1", &params)? {
        1 => ok("success".into()), // TODO update
        0 => bad_request(),
        _ => internal_server_error(),
    }
}

/// A simple Spin HTTP component.
#[http_component]
fn episode_api(req: Request) -> Result<Response> {
    let uri = spin_sdk::config::get("postgres_uri")?;
    let api: Api<RawEpisode> = get_api_from_request(req)?;

    match api {
        Api::Create(model) => create(&uri, model),
        Api::FindAll => find_all(&uri),
        Api::FindById(id) => find_one(&uri, id),
        Api::Update(id, model) => update(&uri, id, model),
        Api::Delete(id) => delete(&uri, id),
        Api::MethodNotAllowed => method_not_allowed(),
        Api::NotFound => not_found(),
        Api::BadRequest => bad_request(),
        Api::InternalServerError => internal_server_error(),
    }

    // let response = ...;
    // match response {
    //     Ok(response) => Ok(response),
    //     Err(_) => internal_server_error(),
    // }
}
