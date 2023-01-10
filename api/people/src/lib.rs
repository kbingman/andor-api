use anyhow::Result;
use rest_api::api::{get_api_from_request, Api};
use rest_api::handlers::{bad_request, internal_server_error, method_not_allowed, not_found, ok};
use spin_sdk::{
    http::{Request, Response},
    http_component,
    pg::{self, ParameterValue},
};

use crate::models::{aggregate_people, as_person, Payload, Person, RawPerson};

mod models;

fn update_episode_ids(uri: &str, person_id: i32, ids: &Vec<i32>) -> Vec<i32> {
    let mut episode_ids: Vec<i32> = Vec::new();
    for episode_id in ids {
        let sql = "
            INSERT INTO 
                people_episodes (person_id, episode_id) 
            VALUES ($1, $2)
        ";
        let params = [
            ParameterValue::Int32(person_id),
            ParameterValue::Int32(*episode_id),
        ];
        match pg::query(uri, sql, &params) {
            Ok(_) => episode_ids.push(*episode_id),
            Err(err) => println!("Error: {:#?}", err),
        }
    }

    episode_ids
}

/// Creates one record
pub(crate) fn create(uri: &str, model: RawPerson) -> Result<Response> {
    let sql = "INSERT INTO people (name) VALUES ($1) RETURNING id, name";
    let params = vec![ParameterValue::Str(&model.name)];
    let rowset = pg::query(uri, sql, &params)?;

    match rowset.rows.first() {
        Some(row) => {
            let person = &as_person(row)?;
            let episode_ids = update_episode_ids(uri, person.id, &model.episode_ids);

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
    let sql = "
        SELECT 
            people.id, 
            people.name, 
            people_episodes.episode_id
        FROM people
        LEFT join people_episodes on (people.id = people_episodes.person_id)
    ";
    let rowset = pg::query(uri, sql, &[])?;
    let results = aggregate_people(rowset);

    ok(serde_json::to_string(&Payload { results: &results? })?)
}

/// Finds one record by ID
pub(crate) fn find_one(address: &str, id: i32) -> Result<Response> {
    let sql = "
        SELECT 
            people.id, 
            people.name, 
            people_episodes.episode_id
        FROM people 
        LEFT join people_episodes on (people.id = people_episodes.person_id)
        WHERE people.id=$1
    ";
    let params = vec![ParameterValue::Int32(id)];
    let rowset = pg::query(address, sql, &params)?;
    let results = aggregate_people(rowset)?;

    match results.first() {
        Some(person) => ok(serde_json::to_string(&person)?),
        None => not_found(),
    }
}

/// Updates one record by ID
pub(crate) fn update(uri: &str, id: i32, model: RawPerson) -> Result<Response> {
    let sql = "
        UPDATE 
            people 
        SET 
            name=$2 
        WHERE id=$1 
        RETURNING  
            people.id, people.name
    ";
    let params = vec![ParameterValue::Int32(id), ParameterValue::Str(&model.name)];
    let rowset = pg::query(uri, sql, &params)?;

    match rowset.rows.first() {
        Some(row) => {
            let person = &as_person(row)?;
            let episode_ids = update_episode_ids(uri, person.id, &model.episode_ids);

            println!("Episode IDs {:#?}", episode_ids);

            ok(serde_json::to_string(&Person {
                name: person.name.to_owned(),
                id: person.id,
                episode_ids,
            })?)
        }
        None => not_found(),
    }
}

pub(crate) fn delete(address: &str, id: i32) -> Result<Response> {
    let params = vec![ParameterValue::Int32(id)];
    let sql = "
        SELECT 
            people.id, 
            people.name, 
            people_episodes.episode_id
        FROM people 
        LEFT join people_episodes on (people.id = people_episodes.person_id)
        WHERE people.id=$1
    ";
    let rowset = pg::query(address, sql, &params)?;
    let results = aggregate_people(rowset)?;
    
    match results.first() {
        Some(person) => {
            for episode_id in &person.episode_ids {
                pg::execute(
                    address, 
                    "DELETE FROM people_episodes WHERE id=$1", 
                    &vec![ParameterValue::Int32(*episode_id)],
                )?;
            }
            match pg::execute(address, "DELETE FROM people WHERE id=$1", &params)? {
                1 => ok("success".into()), // TODO update
                0 => bad_request(),
                _ => internal_server_error(),
            }
        },
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
