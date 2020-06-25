use super::validation_error;
use validator::ValidationError;

pub type TProcessPriority = String;

#[allow(clippy::match_same_arms)] // Readability.
pub fn validate_process_priority(process_priority: &str) -> Result<(), ValidationError> {
    match process_priority {
        "normal" => Ok(()),
        "above-normal" => Ok(()),
        "high" => Ok(()),
        _ => Err(validation_error("unknown_value",
                                  "Invalid process priority."))
    }
}
