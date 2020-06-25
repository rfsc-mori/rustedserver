use super::validation_error;
use validator::ValidationError;

pub type THousePrice = i32;

pub fn validate_house_price(house_price: &THousePrice) -> Result<(), ValidationError> {
    match *house_price {
        price if price >= 0 => Ok(()),
        -1 => Ok(()),
        _ => Err(validation_error("invalid_range",
                                  "House price allowed range: `>= 0` || `-1`."))
    }
}
