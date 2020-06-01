use serde::{Serialize, Deserialize};

#[derive(Default, Serialize, Deserialize)]
pub struct Map {
    map_name: String,
    map_author: String,
}
