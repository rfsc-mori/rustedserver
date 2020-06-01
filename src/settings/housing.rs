use serde::{Serialize, Deserialize};

#[derive(Default, Serialize, Deserialize)]
pub struct Housing {
    pub house_price_each_sqm: i64,
    pub house_rent_period: String,
    pub house_owned_by_account: bool,
}
