use super::validation_error;
use validator::ValidationError;

pub type TMapName = String;

pub fn validate_map_name(map_name: &str) -> Result<(), ValidationError> {
    match map_name {
        name if name.is_empty() => {
            Err(validation_error("empty_string",
                                 "Invalid map name."))
        },
        name if name.ends_with(".otbm") => {
            Err(validation_error("invalid_map_name",
                                 "The map name must not have .otbm at the end."))
        },
        _ => Ok(())
    }
}
