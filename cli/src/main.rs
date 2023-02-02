use std::fs::File;

use anyhow::Result;
use clap::Parser;
use csv::StringRecord;
use reqwest::StatusCode;
use serde_json::{Map, Value};

use crate::records::{as_episode, as_person};

mod records;

/// A simple CLI for importing data into the Andor API app
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the CSV to import
    #[arg(short, long)]
    path: String,

    /// Name of the table to update
    #[arg(short, long)]
    table: String,
}

fn as_record(table: &str, record: StringRecord) -> Map<String, Value> {
    match table {
        "people" => as_person(record),
        "episodes" => as_episode(record),
        _ => Map::new(),
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    let client = reqwest::Client::new();
    let table = args.table;

    let file = File::open(args.path)?;
    let mut reader = csv::ReaderBuilder::new()
        .trim(csv::Trim::All)
        .from_reader(file);

    for result in reader.records() {
        let params = as_record(&table, result?);
        let url = "http://localhost:3000";

        let res = client
            .post(format!("{}/api/{}", &url, &table))
            .json(&params)
            .send()
            .await?;

        match res.status() {
            StatusCode::OK => println!("Created record successfully"),
            StatusCode::BAD_REQUEST => println!("Record exists"),
            _ => println!("Error creating record"),
        }
    }

    Ok(())
}
