use serde::{Serialize, Deserialize};

#[derive(Default, Serialize, Deserialize)]
pub struct Monsters {
    despawn_range: i64,
    despawn_radius: i64,
}
