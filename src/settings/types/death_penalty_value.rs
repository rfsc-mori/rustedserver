use super::validation_error;
use validator::ValidationError;

pub type TDeathPenaltyValue = i32;

pub fn validate_death_penalty_value(death_penalty_value: &TDeathPenaltyValue)
    -> Result<(), ValidationError> {
    match *death_penalty_value {
        death_penalty_value if death_penalty_value >= 0 => Ok(()),
        -1 => Ok(()),
        _ => Err(validation_error("invalid_range",
                                  "Death penalty value allowed range: `>= 0` || `-1`."))
    }
}
