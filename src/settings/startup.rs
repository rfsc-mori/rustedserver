use super::types::*;
use serde::{Serialize, Deserialize};
use validator::Validate;
use validator_derive::Validate;

#[derive(Debug, Default, Serialize, Deserialize, Validate)]
pub struct StartupOptions {
    #[validate(custom = "validate_process_priority")]
    pub default_priority: TProcessPriority,
    pub startup_database_optimization: bool,
}
