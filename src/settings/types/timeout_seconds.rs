use super::validation_error;
use validator::ValidationError;

pub type TTimeoutSeconds = i64;

pub fn validate_timeout_seconds(timeout_seconds: &TTimeoutSeconds) -> Result<(), ValidationError> {
    match *timeout_seconds {
        timeout_seconds if timeout_seconds >= 0 => Ok(()),
        _ => Err(validation_error("invalid_range",
                                  "Timeout seconds allowed range: `>= 0`."))
    }
}
