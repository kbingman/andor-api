use csv::StringRecord;
use serde_json::{Map, Value};

/// Converts the Person CSV row into a JSON Map
pub fn as_person(record: StringRecord) -> Map<String, Value> {
    let episode_ids = if record[3] == "".to_string() {
        Vec::new()
    } else {
        record[3]
            .split(",")
            .collect::<Vec<&str>>()
            .iter()
            .map(|id| Value::Number(id.parse().unwrap()))
            .collect()
    };

    vec![
        ("name".to_string(), Value::String(record[1].to_string())),
        (
            "description".to_string(),
            Value::String(record[2].to_string()),
        ),
        ("episodeIds".to_string(), Value::Array(episode_ids)),
    ]
    .into_iter()
    .collect()
}

/// Converts the Episode CSV row into a JSON Map
pub fn as_episode(record: StringRecord) -> Map<String, Value> {
    vec![
        (
            "episode".to_string(),
            Value::Number(record[1].parse().unwrap()),
        ),
        ("title".to_string(), Value::String(record[2].to_string())),
        (
            "description".to_string(),
            Value::String(record[3].to_string()),
        ),
        ("peopleIds".to_string(), Value::Array(vec![])),
    ]
    .into_iter()
    .collect()
}
