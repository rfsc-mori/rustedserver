use serde::{Serialize, Deserialize};

#[derive(Default, Serialize, Deserialize)]
pub struct Rates {
    pub rate_exp: i64,
    pub rate_skill: i64,
    pub rate_loot: i64,
    pub rate_magic: i64,
    pub rate_spawn: i64,
}
