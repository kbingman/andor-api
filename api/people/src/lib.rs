use anyhow::Result;

use rest_api::api::{get_api_from_request, Api};
use rest_api::handlers::{bad_request, internal_server_error, method_not_allowed, not_found, ok};
use spin_sdk::{
    http::{Request, Response},
    http_component,
};

use crate::models::{aggregate_people, as_person, Payload, Person, RawPerson};

mod db;
mod models;

/// Creates one record
pub(crate) fn create(uri: &str, model: RawPerson) -> Result<Response> {
    let rowset = db::insert_person(uri, &model)?;

    match rowset.rows.first() {
        Some(row) => {
            let person = &as_person(row)?;
            let episode_ids = db::insert_episode_ids(uri, person.id, &model.episode_ids);

            ok(serde_json::to_string(&Person {
                name: person.name.to_owned(),
                id: person.id,
                episode_ids,
            })?)
        }
        None => not_found(),
    }
}

/// Finds all People in the DB
pub(crate) fn find_all(uri: &str) -> Result<Response> {
    let rowset = db::find_all_people(uri)?;
    let results = aggregate_people(rowset);

    ok(serde_json::to_string(&Payload { results: &results? })?)
}

/// Finds one record by ID
pub(crate) fn find_one(uri: &str, id: i32) -> Result<Response> {
    let rowset = db::find_one_person(uri, id)?;
    let results = aggregate_people(rowset)?;

    match results.first() {
        Some(person) => ok(serde_json::to_string(&person)?),
        None => not_found(),
    }
}

/// Updates one record by ID
pub(crate) fn update(uri: &str, id: i32, model: RawPerson) -> Result<Response> {
    let rowset = db::update_person(uri, id, &model)?;

    match rowset.rows.first() {
        Some(row) => {
            let person = &as_person(row)?;
            let episode_ids = db::insert_episode_ids(uri, person.id, &model.episode_ids);

            ok(serde_json::to_string(&Person {
                name: person.name.to_owned(),
                id: person.id,
                episode_ids,
            })?)
        }
        None => not_found(),
    }
}

pub(crate) fn delete(uri: &str, id: i32) -> Result<Response> {
    let rowset = db::find_one_person(uri, id)?;
    let results = aggregate_people(rowset)?;

    match results.first() {
        Some(person) => {
            db::delete_episode_ids(uri, person.id)?;
            match db::delete_person(uri, id)? {
                1 => ok("success".into()), // TODO update
                0 => bad_request(),
                _ => internal_server_error(),
            }
        }
        None => not_found(),
    }
}

/// A Spin REST component.
#[http_component]
fn people_api(req: Request) -> Result<Response> {
    let uri = spin_sdk::config::get("postgres_uri")?;
    let api: Api<RawPerson> = get_api_from_request(req)?;

    // let response =
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

    // match response {
    //     Ok(response) => Ok(response),
    //     Err(_) => internal_server_error(),
    // }
}
