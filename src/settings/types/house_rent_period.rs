use super::validation_error;
use validator::ValidationError;

pub type THouseRentPeriod = String;

#[allow(clippy::match_same_arms)] // Readability.
pub fn validate_house_rent_period(house_rent_period: &str) -> Result<(), ValidationError> {
    match house_rent_period {
        "yearly" => Ok(()),
        "monthly" => Ok(()),
        "weekly" => Ok(()),
        "daily" => Ok(()),
        "never" => Ok(()),
        _ => Err(validation_error("unknown_value",
                                  "Invalid house rent period."))
    }
}
