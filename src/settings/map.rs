use serde::{Serialize, Deserialize};

#[derive(Default, Serialize, Deserialize)]
pub struct Map {
    pub map_name: String,
    pub map_author: String,
}
