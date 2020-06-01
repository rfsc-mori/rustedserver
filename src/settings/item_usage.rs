use serde::{Serialize, Deserialize};

#[derive(Default, Serialize, Deserialize)]
pub struct ItemUsage {
    pub time_between_actions: i64,
    pub time_between_ex_actions: i64,
}
