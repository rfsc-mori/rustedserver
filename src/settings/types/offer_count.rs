use super::validation_error;
use validator::ValidationError;

pub type TOfferCount = u32;

pub fn validate_offer_count(offer_count: &TOfferCount) -> Result<(), ValidationError> {
    match *offer_count {
        count if count > 0 => Ok(()),
        0 => Ok(()),
        _ => Err(validation_error("invalid_range",
                                  "Offer count allowed range: `>= 0`."))
    }
}
