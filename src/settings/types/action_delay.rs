use super::validation_error;
use validator::ValidationError;

pub type TActionDelay = i64;

pub fn validate_action_delay(action_delay: &TActionDelay) -> Result<(), ValidationError> {
    match *action_delay {
        action_delay if action_delay >= 0 => Ok(()),
        _ => Err(validation_error("invalid_range",
                                  "Action delay allowed range: `>= 0`."))
    }
}
