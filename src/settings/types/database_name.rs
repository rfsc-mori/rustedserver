use super::validation_error;
use validator::ValidationError;

pub type TDatabaseName = String;

pub fn validate_database_name(database_name: &TDatabaseName) -> Result<(), ValidationError> {
    match database_name {
        database_name if database_name.len() > 0 => Ok(()),
        _ => Err(validation_error("empty_string",
                                  "Invalid database name."))
    }
}
