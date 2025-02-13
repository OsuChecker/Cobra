use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MapSet {
    pub maps: Vec<Map>,
    pub cover: String,
    pub song: String,
    pub author: String,
    pub creator: String,
    pub link: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Map {
    pub difficulty: f64,
    pub pattern: String,
}