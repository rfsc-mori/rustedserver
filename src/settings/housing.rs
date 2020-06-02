use super::types::*;
use serde::{Serialize, Deserialize};
use validator::Validate;
use validator_derive::Validate;

#[derive(Debug, Default, Serialize, Deserialize, Validate)]
pub struct Housing {
    #[validate(custom = "validate_house_price")]
    pub house_price_each_sqm: THousePrice,
    #[validate(custom = "validate_house_rent_period")]
    pub house_rent_period: THouseRentPeriod,
    pub house_owned_by_account: bool,
}
