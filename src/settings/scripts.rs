use serde::{Serialize, Deserialize};
use validator::Validate;
use validator_derive::Validate;

#[derive(Debug, Default, Serialize, Deserialize, Validate)]
pub struct Scripts {
    pub warn_unsafe_scripts: bool,
    pub convert_unsafe_scripts: bool,
}
