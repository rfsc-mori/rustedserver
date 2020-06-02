use super::validation_error;
use validator::ValidationError;

pub type TProcessPriority = String;

pub fn validate_process_priority(process_priority: &TProcessPriority)
    -> Result<(), ValidationError> {
    match process_priority.as_str() {
        "normal" => Ok(()),
        "above-normal" => Ok(()),
        "high" => Ok(()),
        _ => Err(validation_error("unknown_value",
                                  "Invalid process priority."))
    }
}
