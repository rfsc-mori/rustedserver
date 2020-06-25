use super::validation_error;
use validator::ValidationError;

pub type TIpAddress = String;

pub fn validate_ip_address(ip_address: &str) -> Result<(), ValidationError> {
    match ip_address {
        ip if !ip.is_empty() => Ok(()),
        _ => Err(validation_error("empty_string",
                                  "Invalid ip address."))
    }
}
