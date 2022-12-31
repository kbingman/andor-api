use anyhow::Result;
use spin_sdk::{
    http::{Request, Response},
    http_component,
    pg::{self, ParameterValue}
};
use crate::api::{get_api_from_request, Api};
use crate::response::{
    bad_request,
    internal_server_error,
    method_not_allowed,
    not_found,
    ok,
};
use crate::models::{Payload, Person, RawPerson};

mod api;
mod models;
mod response;
mod util;

/// Creates one record
pub(crate) fn create(address: &str, model: RawPerson) -> Result<Response> {
    let sql = "INSERT INTO people (name) VALUES ($1) RETURNING id, name";
    let params = vec![
        ParameterValue::Str(&model.name)
    ];
    
    let rowset = pg::query(&address, sql, &params)?;
 
    match rowset.rows.first() {
        Some(row) => ok(serde_json::to_string(&Person::from_row(&row)?)?),
        None => not_found(),
    }
}

/// Finds all People in the DB
pub(crate) fn find_all(address: &str) -> Result<Response> {
    let sql = "SELECT id, name, created_at FROM people";
    let rowset = pg::query(&address, sql, &[])?;
    
    let mut results: Vec<Person> = Vec::new();
    for row in rowset.rows {
        results.push(Person::from_row(&row)?);
    }
        
    ok(serde_json::to_string(&Payload { 
        results
    })?)
}

/// Finds one record by ID
pub(crate) fn find_one(address: &str, id: i32) -> Result<Response> {
    let sql = "SELECT id, name FROM people WHERE id=$1";
    let params = vec![ParameterValue::Int32(id)];
    let rowset = pg::query(&address, &sql, &params)?;
 
    match rowset.rows.first() {
        Some(row) => ok(serde_json::to_string(&Person::from_row(&row)?)?),
        None => not_found(),
    }
}

/// Updates one record by ID
pub(crate) fn update(address: &str, id: i32, model: RawPerson) -> Result<Response> {
    let sql = "UPDATE people SET name=$2 WHERE id=$1 RETURNING id, name";
    let params = vec![
        ParameterValue::Int32(id), 
        ParameterValue::Str(&model.name)
    ];
    let rowset = pg::query(&address, &sql, &params)?;
 
    match rowset.rows.first() {
        Some(row) => ok(serde_json::to_string(&Person::from_row(&row)?)?),
        None => not_found(),
    }
}

pub(crate) fn delete(address: &str, id: i32) -> Result<Response> {
    let sql = "DELETE FROM people WHERE id=$1";
    let params = vec![
        ParameterValue::Int32(id), 
    ];
    
    match pg::execute(&address, &sql, &params)? {
        1 => ok("success".into()), // TODO update
        0 => bad_request(),
        _ => internal_server_error(),
    }
}

/// A Spin REST component.
#[http_component]
fn people_api(req: Request) -> Result<Response> {
    let uri = spin_sdk::config::get("postgres_uri")?;
    let api: Api<RawPerson> = get_api_from_request(req)?;
    
    let response = match api {
        Api::Create(model) => create(&uri, model),
        Api::FindAll => find_all(&uri),
        Api::FindById(id) => find_one(&uri, id),
        Api::Update(id, model) => update(&uri, id, model),
        Api::Delete(id) => delete(&uri, id),
        Api::MethodNotAllowed => method_not_allowed(),
        Api::NotFound => not_found(),
        Api::BadRequest => bad_request(),
        Api::InternalServerError => internal_server_error(),
    };

    match response {
        Ok(response) => Ok(response),
        Err(_) => internal_server_error(),
    }
}
