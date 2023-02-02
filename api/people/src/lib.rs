use anyhow::Result;

use rest_api::api::{get_api_from_request, Api};
use rest_api::handlers::{bad_request, internal_server_error, method_not_allowed, not_found, ok};
use spin_sdk::{
    http::{Request, Response},
    http_component,
};

use crate::db::{DbAdapter, PeopleDb};
use crate::models::Person;

mod db;
mod models;

/// Creates one record
pub(crate) fn create<Db: DbAdapter<Person>>(db: Db, model: Person) -> Result<Response> {
    match db.insert(&model)? {
        Some(person) => ok(serde_json::to_string(&person)?),
        None => not_found(),
    }
}

/// Finds all People in the DB
pub(crate) fn find_all<Db: DbAdapter<Person>>(db: Db) -> Result<Response> {
    let results = db.find_all()?;

    ok(serde_json::to_string(&results)?)
}

/// Finds one record by ID
pub(crate) fn find_one<Db: DbAdapter<Person>>(db: Db, id: i32) -> Result<Response> {
    let person = db.find_one(id)?;

    match person {
        Some(person) => ok(serde_json::to_string(&person)?),
        None => not_found(),
    }
}

/// Updates one record by ID
pub(crate) fn update<Db: DbAdapter<Person>>(db: Db, id: i32, model: Person) -> Result<Response> {
    match db.update(id, &model)? {
        Some(person) => ok(serde_json::to_string(&person)?),
        None => not_found(),
    }
}

pub(crate) fn delete<Db: DbAdapter<Person>>(db: Db, id: i32) -> Result<Response> {
    match db.delete(id)? {
        1 => ok("success".into()), // TODO update
        0 => bad_request(),
        _ => internal_server_error(),
    }
}

/// A Spin REST component.
#[http_component]
fn people_api(req: Request) -> Result<Response> {
    let uri = spin_sdk::config::get("postgres_uri")?;
    let api: Api<Person> = get_api_from_request(req)?;
    let people_db = PeopleDb::new(uri);

    match api {
        Api::Create(model) => create(people_db, model),
        Api::FindAll => find_all(people_db),
        Api::FindById(id) => find_one(people_db, id),
        Api::Update(id, model) => update(people_db, id, model),
        Api::Delete(id) => delete(people_db, id),
        Api::MethodNotAllowed => method_not_allowed(),
        Api::NotFound => not_found(),
        Api::BadRequest => bad_request(),
        Api::InternalServerError => internal_server_error(),
    }
}
