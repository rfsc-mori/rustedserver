use super::validation_error;
use validator::ValidationError;

pub type TRate = u64;

#[allow(unused_comparisons)] // `>= 0` covers u64 range.
pub fn validate_rate(rate: &TRate) -> Result<(), ValidationError> {
    match *rate {
        rate if rate >= 0 => Ok(()),
        _ => Err(validation_error("invalid_range",
                                  "Rate allowed range: `>= 0`."))
    }
}
