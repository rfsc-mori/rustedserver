use super::types::*;
use serde::{Serialize, Deserialize};
use validator::Validate;
use validator_derive::Validate;

#[derive(Debug, Default, Serialize, Deserialize, Validate)]
pub struct MapOptions {
    #[validate(custom = "validate_map_name")]
    pub map_name: TMapName,
    pub map_author: String,
}
