use super::validation_error;
use validator::ValidationError;

pub type TKillCount = i64;

pub fn validate_kill_count(kill_count: &TKillCount) -> Result<(), ValidationError> {
    match *kill_count {
        kill_count if kill_count > 0 => Ok(()),
        0 => Ok(()),
        _ => Err(validation_error("invalid_range",
                                  "Kill count allowed range: `>= 0`."))
    }
}
