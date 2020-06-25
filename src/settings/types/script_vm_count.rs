use super::validation_error;
use validator::ValidationError;

pub type TScriptVMCount = usize;

pub fn validate_script_vm_count(vm_count: &TScriptVMCount) -> Result<(), ValidationError> {
    match *vm_count {
        count if count > 0 => Ok(()),
        0 => Ok(()),
        _ => Err(validation_error("invalid_range",
                                  "Script VM count allowed range: `> 0 || 0`."))
    }
}
