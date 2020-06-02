use super::validation_error;
use validator::ValidationError;

pub type THostAddress = String;

pub fn validate_host_address(host_address: &THostAddress) -> Result<(), ValidationError> {
    match host_address {
        host_address if host_address.len() > 0 => Ok(()),
        _ => Err(validation_error("empty_string",
                                  "Invalid host address."))
    }
}
