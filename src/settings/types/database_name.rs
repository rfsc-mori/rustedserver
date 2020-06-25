use super::validation_error;
use validator::ValidationError;

pub type TDatabaseName = String;

pub fn validate_database_name(database_name: &str) -> Result<(), ValidationError> {
    match database_name {
        name if !name.is_empty() => Ok(()),
        _ => Err(validation_error("empty_string",
                                  "Invalid database name."))
    }
}
