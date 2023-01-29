use anyhow::Result;
use spin_sdk::pg::{self, ParameterValue};

use crate::models::{aggregate_people, as_person, Person};
use db_adapter::DbAdapter;

pub(crate) struct PeopleDb {
    uri: String,
}

impl PeopleDb {
    pub(crate) fn new(uri: String) -> Self {
        Self { uri }
    }
}

impl DbAdapter<Person> for PeopleDb {
    fn insert(&self, model: &Person) -> Result<Option<Person>> {
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
        let rowset = pg::query(&self.uri, sql, &params)?;

        Ok(match rowset.rows.first() {
            Some(row) => {
                let person = as_person(row)?;
                let episode_ids = insert_episode_ids(&self.uri, person.id, &model.episode_ids);

                Some(Person {
                    episode_ids,
                    ..person.to_owned()
                })
            }
            _ => None,
        })
    }

    fn find_all(&self) -> Result<Vec<Person>> {
        let sql = "
            SELECT 
                people.id, 
                people.name, 
                people.description,
                people_episodes.episode_id
            FROM people
            LEFT JOIN people_episodes on (people.id = people_episodes.person_id)
        ";
        let rowset = pg::query(&self.uri, sql, &[])?;

        Ok(aggregate_people(rowset)?)
    }

    fn find_one(&self, id: i32) -> Result<Option<Person>> {
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

        let rowset = pg::query(&self.uri, sql, &[ParameterValue::Int32(id)])?;
        let results = aggregate_people(rowset)?;

        Ok(match results.first() {
            Some(person) => Some(person.clone()),
            _ => None,
        })
    }

    fn update(&self, id: i32, model: &Person) -> Result<Option<Person>> {
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
        let rowset = pg::query(&self.uri, sql, &params)?;

        Ok(match rowset.rows.first() {
            Some(row) => {
                let person = as_person(row)?;
                let episode_ids = insert_episode_ids(&self.uri, person.id, &model.episode_ids);

                Some(Person {
                    episode_ids,
                    ..person.to_owned()
                })
            },
            _ => None,
        })
    }

    /// Delete one person by ID
    fn delete(&self, id: i32) -> Result<u64> {
        let result = pg::execute(
            &self.uri,
            "DELETE FROM people WHERE id=$1",
            &[ParameterValue::Int32(id)],
        )?;

        Ok(result)
    }
}

/// Update episode_ids per person
pub(crate) fn insert_episode_ids(
    uri: &str,
    person_id: Option<i32>,
    episode_ids: &Vec<i32>,
) -> Vec<i32> {
    match person_id {
        Some(person_id) => {
            let mut ids: Vec<i32> = Vec::new();
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
                    Ok(_) => ids.push(*episode_id),
                    Err(err) => println!("Error: {:#?}", err),
                }
            }
            ids
        }
        None => Vec::new(),
    }
}
