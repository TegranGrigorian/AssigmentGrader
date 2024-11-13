use std::{collections::HashMap, fs, io};
use serde::{Deserialize, Serialize};
use serde_json;
use ordered_float::OrderedFloat;

#[derive(Serialize, Deserialize)]
pub struct Data {
    pub classes: HashMap<String, OrderedFloat<f64>>,
    pub sub_topics: HashMap<i64, Vec<(String, String, String, f64)>>,
    pub sub_topic_constants: HashMap<i64, Vec<(String, f64)>>,
    pub sub_topic_grades: HashMap<i64, Vec<(String, OrderedFloat<f64>)>>, // Store subtopic grades
}

pub fn save_data(file_path: &str, data: &Data) -> Result<(), std::io::Error> {
    let serialized_data = serde_json::to_string_pretty(data)?;
    fs::write(file_path, serialized_data)?;
    Ok(())
}

pub fn load_data(file_path: &str) -> Result<Data, std::io::Error> {
    let data = fs::read_to_string(file_path)?;
    let parsed_data: Data = serde_json::from_str(&data).unwrap_or(Data {
        classes: HashMap::new(),
        sub_topics: HashMap::new(),
        sub_topic_constants: HashMap::new(),
        sub_topic_grades: HashMap::new(),
    });
    Ok(parsed_data)
}
