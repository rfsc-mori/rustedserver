use super::types::*;
use serde::{Serialize, Deserialize};
use validator::Validate;
use validator_derive::Validate;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct ScriptOptions {
    #[validate(custom = "validate_script_vm_count")]
    pub script_vm_count: TScriptVMCount,
    pub warn_unsafe_scripts: bool,
    pub convert_unsafe_scripts: bool,
}
