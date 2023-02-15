use anyhow::Result;
use serde::{Deserialize, Serialize};
use spin_sdk::pg::{Decode, Row, RowSet};
use std::collections::HashMap;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Person {
    pub id: Option<i32>,
    pub name: String,
    pub description: String,
    pub episode_ids: Vec<i32>,
}

#[derive(Deserialize, Serialize)]
pub(crate) struct Payload<T> {
    pub results: T,
}

pub(crate) fn as_person(row: &Row) -> Result<Person> {
    let id = i32::decode(&row[0])?;
    let name = String::decode(&row[1])?;
    let description = String::decode(&row[2])?;

    let episode_ids: Vec<i32> = if row.len() >= 4 {
        match i32::decode(&row[3]) {
            Ok(id) => vec![id],
            _ => Vec::new(),
        }
    } else {
        Vec::new()
    };

    Ok(Person {
        id: Some(id),
        name,
        description,
        episode_ids,
    })
}

/// The current DB query returns a row for every Person and Episode ID combination
/// This takes a Vector of RowSets and aggregates it into unique records (by ID)
pub(crate) fn aggregate_people(rowset: RowSet) -> Result<Vec<Person>> {
    let hashmap: Result<HashMap<i32, Person>> =
        rowset
            .rows
            .iter()
            .try_fold(HashMap::<i32, Person>::new(), |mut acc, row| {
                let mut person = as_person(row)?;

                if let Some(id) = person.id {
                    match acc.get(&id) {
                        Some(record) => {
                            person.episode_ids.append(&mut record.episode_ids.clone());
                            person.episode_ids.sort();
                            acc.insert(id, person);
                        }
                        None => {
                            acc.insert(id, person);
                        }
                    }
                }

                Ok(acc)
            });

    let mut people: Vec<Person> = hashmap?.values().cloned().collect();
    people.sort_by(|a, b| a.id.cmp(&b.id));

    Ok(people)
}
