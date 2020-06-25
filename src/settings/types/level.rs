use super::validation_error;
use validator::ValidationError;

pub type TLevel = u32;

#[allow(unused_comparisons)] // `>= 0` covers u32 range.
pub fn validate_level(player_level: &TLevel) -> Result<(), ValidationError> {
    match *player_level {
        level if level >= 0 => Ok(()),
        _ => Err(validation_error("invalid_range",
                                  "Level allowed range: `>= 0`."))
    }
}
