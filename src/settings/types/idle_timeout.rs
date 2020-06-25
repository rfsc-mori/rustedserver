use super::validation_error;
use validator::ValidationError;

pub type TIdleTimeout = u32;

#[allow(unused_comparisons)] // `>= 0` covers u32 range.
pub fn validate_idle_timeout(idle_timeout: &TIdleTimeout) -> Result<(), ValidationError> {
    match *idle_timeout {
        timeout if timeout >= 0 => Ok(()),
        _ => Err(validation_error("invalid_range",
                                  "Idle timeout allowed range: `>= 0`."))
    }
}
