use super::types::*;
use serde::{Serialize, Deserialize};
use validator::Validate;
use validator_derive::Validate;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct MarketOptions {
    #[validate(custom = "validate_offer_duration")]
    pub market_offer_duration: TOfferDuration,
    pub premium_to_create_market_offer: bool,
    #[validate(custom = "validate_offer_expired_interval")]
    pub check_expired_market_offers_each_minutes: TOfferExpiredInterval,
    #[validate(custom = "validate_offer_count")]
    pub max_market_offers_at_a_time_per_player: TOfferCount,
}
