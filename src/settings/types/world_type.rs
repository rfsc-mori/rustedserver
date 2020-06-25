use super::validation_error;
use validator::ValidationError;

pub type TWorldType = String;

#[allow(clippy::match_same_arms)] // Readability.
pub fn validate_world_type(world_type: &str) -> Result<(), ValidationError> {
    match world_type {
        "pvp" => Ok(()),
        "no-pvp" => Ok(()),
        "pvp-enforced" => Ok(()),
        _ => Err(validation_error("unknown_value",
                                  "Invalid world type."))
    }
}
