use super::validation_error;
use validator::ValidationError;

pub type TOfferDuration = u32;

#[allow(unused_comparisons)] // `>= 0` covers u32 range.
pub fn validate_offer_duration(offer_duration: &TOfferDuration) -> Result<(), ValidationError> {
    match *offer_duration {
        duration if duration >= 0 => Ok(()),
        _ => Err(validation_error("invalid_range",
                                  "Offer duration allowed range: `>= 0`."))
    }
}
