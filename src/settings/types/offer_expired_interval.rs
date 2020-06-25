use super::validation_error;
use validator::ValidationError;

pub type TOfferExpiredInterval = u32;

pub fn validate_offer_expired_interval(i: &TOfferExpiredInterval) -> Result<(), ValidationError> {
    match *i {
        offer_expired_interval if offer_expired_interval > 0 => Ok(()),
        0 => Ok(()),
        _ => Err(validation_error("invalid_range",
                                  "Offer expired interval allowed range: `>= 0`."))
    }
}
