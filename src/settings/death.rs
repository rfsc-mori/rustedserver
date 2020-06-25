use super::types::*;
use serde::{Serialize, Deserialize};
use validator::Validate;
use validator_derive::Validate;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct DeathOptions {
    #[validate(custom = "validate_death_penalty_value")]
    pub death_lose_percent: TDeathPenaltyValue,
}
