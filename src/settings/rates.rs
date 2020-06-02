use super::types::*;
use serde::{Serialize, Deserialize};
use validator::Validate;
use validator_derive::Validate;

#[derive(Debug, Default, Serialize, Deserialize, Validate)]
pub struct Rates {
    #[validate(custom = "validate_rate")]
    pub rate_exp: TRate,
    #[validate(custom = "validate_rate")]
    pub rate_skill: TRate,
    #[validate(custom = "validate_rate")]
    pub rate_loot: TRate,
    #[validate(custom = "validate_rate")]
    pub rate_magic: TRate,
    #[validate(custom = "validate_rate")]
    pub rate_spawn: TRate,
}
