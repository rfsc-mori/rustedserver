use serde::{Serialize, Deserialize};

#[derive(Default, Serialize, Deserialize)]
pub struct Death {
    death_lose_percent: i64,
}
