use super::validation_error;
use validator::ValidationError;

pub type TIpAddress = String;

pub fn validate_ip_address(ip_address: &TIpAddress) -> Result<(), ValidationError> {
    match ip_address {
        ip_address if ip_address.len() > 0 => Ok(()),
        _ => Err(validation_error("empty_string",
                                  "Invalid ip address."))
    }
}
