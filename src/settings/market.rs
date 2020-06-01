use serde::{Serialize, Deserialize};

#[derive(Default, Serialize, Deserialize)]
pub struct Market {
    pub market_offer_duration: i64,
    pub premium_to_create_market_offer: bool,
    pub check_expired_market_offers_each_minutes: i64,
    pub max_market_offers_at_a_time_per_player: i64,
}
