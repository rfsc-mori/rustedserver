use serde::{Serialize, Deserialize};
use validator::Validate;
use validator_derive::Validate;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct StaminaOptions {
    pub stamina_system: bool,
}
