use super::validation_error;
use validator::ValidationError;

pub type TMapName = String;

pub fn validate_map_name(map_name: &TMapName) -> Result<(), ValidationError> {
    match map_name {
        map_name if map_name.is_empty() => {
            Err(validation_error("empty_string",
                                 "Invalid map name."))
        },
        map_name if map_name.ends_with(".otbm") => {
            Err(validation_error("invalid_map_name",
                                 "The map name must not have .otbm at the end."))
        },
        _ => Ok(())
    }
}
