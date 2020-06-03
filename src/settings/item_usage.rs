use super::types::*;
use serde::{Serialize, Deserialize};
use validator::Validate;
use validator_derive::Validate;

#[derive(Debug, Default, Serialize, Deserialize, Validate)]
pub struct ItemUsageOptions {
    #[validate(custom = "validate_action_delay")]
    pub time_between_actions: TActionDelay,
    #[validate(custom = "validate_action_delay")]
    pub time_between_ex_actions: TActionDelay,
}
