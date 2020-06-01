use serde::{Serialize, Deserialize};

#[derive(Default, Serialize, Deserialize)]
pub struct Monsters {
    pub despawn_range: i64,
    pub despawn_radius: i64,
}
