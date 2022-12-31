use anyhow::Result;
use serde::{Deserialize, Serialize};
use spin_sdk::pg::{Decode, Row};

#[derive(Deserialize, Serialize)]
pub(crate) struct Person {
    pub name: String,
    pub id: i32,
}

impl Person {
    pub fn from_row(row: &Row) -> Result<Self> {
        let id = i32::decode(&row[0])?;
        let name = String::decode(&row[1])?;

        Ok(Self { id, name })
    }
}

#[derive(Deserialize, Serialize)]
pub(crate) struct RawPerson {
    pub name: String,
}

#[derive(Deserialize, Serialize)]
pub(crate) struct Payload<T> {
    pub results: T,
}
