use serde::{Serialize, Deserialize};

#[derive(Default, Serialize, Deserialize)]
pub struct Market {
    market_offer_duration: i64,
    premium_to_create_market_offer: bool,
    check_expired_market_offers_each_minutes: i64,
    max_market_offers_at_a_time_per_player: i64,
}
