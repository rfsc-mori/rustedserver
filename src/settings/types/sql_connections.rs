use super::validation_error;
use validator::ValidationError;

pub type TSQLConnections = u32;

#[allow(unused_comparisons)] // `>= 0` covers u32 range.
pub fn validate_sql_connections(sql_connections: &TSQLConnections) -> Result<(), ValidationError> {
    match *sql_connections {
        sql_connections if sql_connections > 0 => Ok(()),
        _ => Err(validation_error("invalid_range",
                                  "SQL connections allowed range: `> 0`."))
    }
}
