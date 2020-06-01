use serde::{Serialize, Deserialize};

#[derive(Default, Serialize, Deserialize)]
pub struct Housing {
    house_price_each_sqm: i64,
    house_rent_period: String,
    house_owned_by_account: bool,
}
