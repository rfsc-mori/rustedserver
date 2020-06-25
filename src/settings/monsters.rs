use super::types::*;
use serde::{Serialize, Deserialize};
use validator::Validate;
use validator_derive::Validate;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct MonsterOptions {
    #[validate(custom = "validate_despawn_range")]
    pub despawn_range: TDespawnRange,
    #[validate(custom = "validate_despawn_range")]
    pub despawn_radius: TDespawnRange,
}
