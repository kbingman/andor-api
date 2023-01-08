use anyhow::Result;
use serde::{Deserialize, Serialize};
use spin_sdk::pg::{Decode, Row, RowSet};
use std::collections::HashMap;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) struct Person {
    pub id: i32,
    pub name: String,
    pub episode_ids: Vec<i32>,
}

#[derive(Deserialize, Serialize)]
pub(crate) struct RawPerson {
    pub name: String,
    pub episode_ids: Vec<i32>,
}

#[derive(Deserialize, Serialize)]
pub(crate) struct Payload<T> {
    pub results: T,
}

pub(crate) fn as_person(row: &Row) -> Result<Person> {
    let id = i32::decode(&row[0])?;
    let name = String::decode(&row[1])?;
    let episode_ids: Vec<i32> = match i32::decode(&row[2]) {
        Ok(id) => vec![id],
        _ => Vec::new(),
    };

    Ok(Person {
        id,
        name,
        episode_ids,
    })
}

pub(crate) fn aggregate_people(rowset: RowSet) -> Result<Vec<Person>> {
    let hashmap: Result<HashMap<i32, Person>> =
        rowset
            .rows
            .iter()
            .try_fold(HashMap::<i32, Person>::new(), |mut acc, row| {
                let mut p = as_person(row)?;

                match acc.get(&p.id) {
                    Some(record) => {
                        p.episode_ids.append(&mut record.episode_ids.clone());
                        acc.insert(p.id, p);
                    }
                    None => {
                        acc.insert(p.id, p);
                    }
                }
                Ok(acc)
            });

    let people: Vec<Person> = hashmap?.values().cloned().collect();

    Ok(people)
}
