use serde::{Serialize, Deserialize};

#[derive(Default, Serialize, Deserialize)]
pub struct ItemUsage {
    time_between_actions: i64,
    time_between_ex_actions: i64,
}
