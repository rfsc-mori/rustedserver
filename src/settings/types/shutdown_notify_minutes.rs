use super::validation_error;
use validator::ValidationError;

pub type TShutdownNotifyMinutes = u64;

#[allow(unused_comparisons)] // `>= 0` covers u64 range.
pub fn validate_shutdown_notify_minutes(shutdown_notify_minutes: &TShutdownNotifyMinutes)
    -> Result<(), ValidationError> {
    match *shutdown_notify_minutes {
        shutdown_notify_minutes if shutdown_notify_minutes >= 0 => Ok(()),
        _ => Err(validation_error("invalid_range",
                                  "Shutdown notify minutes allowed range: `>= 0`."))
    }
}
