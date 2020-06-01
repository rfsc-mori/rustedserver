use serde::{Serialize, Deserialize};

#[derive(Default, Serialize, Deserialize)]
pub struct Rates {
    rate_exp: i64,
    rate_skill: i64,
    rate_loot: i64,
    rate_magic: i64,
    rate_spawn: i64,
}
