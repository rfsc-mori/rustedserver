use super::validation_error;
use validator::ValidationError;

pub type TWorldType = String;

pub fn validate_world_type(world_type: &TWorldType) -> Result<(), ValidationError> {
    match world_type.as_str() {
        "pvp" => Ok(()),
        "no-pvp" => Ok(()),
        "pvp-enforced" => Ok(()),
        _ => Err(validation_error("unknown_value",
                                  "Invalid world type."))
    }
}
