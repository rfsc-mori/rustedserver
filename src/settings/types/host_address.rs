use super::validation_error;
use validator::ValidationError;

pub type THostAddress = String;

pub fn validate_host_address(host_address: &str) -> Result<(), ValidationError> {
    match host_address {
        host if !host.is_empty() => Ok(()),
        _ => Err(validation_error("empty_string",
                                  "Invalid host address."))
    }
}
