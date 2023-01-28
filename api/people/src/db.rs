use anyhow::Result;
use spin_sdk::pg::{self, ParameterValue, PgError, RowSet};

use crate::models::RawPerson;

/// Insert person
pub(crate) fn insert_person(uri: &str, model: &RawPerson) -> Result<RowSet, PgError> {
    let sql = "
        INSERT INTO 
            people (name, description) 
        VALUES ($1, $2) 
        RETURNING id, name, description
    ";
    let params = vec![
        ParameterValue::Str(&model.name),
        ParameterValue::Str(&model.description),
    ];

    pg::query(uri, sql, &params)
}

/// Find all people
pub(crate) fn find_all_people(uri: &str) -> Result<RowSet, PgError> {
    let sql = "
        SELECT 
            people.id, 
            people.name, 
            people.description,
            people_episodes.episode_id
        FROM people
        LEFT JOIN people_episodes on (people.id = people_episodes.person_id)
    ";

    pg::query(uri, sql, &[])
}

/// Find one person by ID
pub(crate) fn find_one_person(uri: &str, id: i32) -> Result<RowSet> {
    let sql = "
        SELECT 
            people.id, 
            people.name, 
            people.description,
            people_episodes.episode_id
        FROM people 
        LEFT JOIN people_episodes on (people.id = people_episodes.person_id)
        WHERE people.id=$1
    ";
    let rowset = pg::query(uri, sql, &[ParameterValue::Int32(id)])?;

    Ok(rowset)
}

/// Update person
pub(crate) fn update_person(uri: &str, id: i32, model: &RawPerson) -> Result<RowSet> {
    let sql = "
        UPDATE 
            people 
        SET 
            name=$2, description=$3
        WHERE id=$1 
        RETURNING  
            people.id, people.name, people.description
    ";
    let params = vec![
        ParameterValue::Int32(id),
        ParameterValue::Str(&model.name),
        ParameterValue::Str(&model.description),
    ];
    let rowset = pg::query(uri, sql, &params)?;

    Ok(rowset)
}

/// Update episode_ids per person
pub(crate) fn insert_episode_ids(uri: &str, person_id: i32, episode_ids: &Vec<i32>) -> Vec<i32> {
    let mut results: Vec<i32> = Vec::new();
    for episode_id in episode_ids {
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
            Ok(_) => results.push(*episode_id),
            Err(err) => println!("Error: {:#?}", err),
        }
    }

    results
}

/// Delete one person by ID
pub(crate) fn delete_person(uri: &str, id: i32) -> Result<u64, spin_sdk::pg::PgError> {
    pg::execute(
        uri,
        "DELETE FROM people WHERE id=$1",
        &[ParameterValue::Int32(id)],
    )
}

/// Delete all people_episodes for a given person by ID
pub(crate) fn delete_episode_ids(uri: &str, person_id: i32) -> Result<u64, spin_sdk::pg::PgError> {
    pg::execute(
        uri,
        "DELETE FROM people_episodes WHERE person_id=$1",
        &[ParameterValue::Int32(person_id)],
    )
}
