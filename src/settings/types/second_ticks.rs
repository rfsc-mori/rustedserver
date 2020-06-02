use super::validation_error;
use validator::ValidationError;

pub type TSecondTicks = i64;

pub fn validate_second_ticks(second_ticks: &TSecondTicks) -> Result<(), ValidationError> {
    match *second_ticks {
        second_ticks if second_ticks >= 0 => Ok(()),
        _ => Err(validation_error("invalid_range",
                                  "Second ticks allowed range: `>= 0`."))
    }
}

pub fn validate_second_ticks_special(second_ticks: &TSecondTicks) -> Result<(), ValidationError> {
    match *second_ticks {
        second_ticks if second_ticks >= 0 => Ok(()),
        -1 => Ok(()),
        _ => Err(validation_error("invalid_range",
                                  "Second ticks (special) allowed range: `>= 0` || `-1`."))
    }
}
