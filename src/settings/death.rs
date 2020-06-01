use serde::{Serialize, Deserialize};

#[derive(Default, Serialize, Deserialize)]
pub struct Death {
    pub death_lose_percent: i64,
}
