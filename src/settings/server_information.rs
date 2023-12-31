use serde::{Serialize, Deserialize};
use validator::Validate;
use validator_derive::Validate;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct ServerInformationOptions {
    pub owner_name: String,
    pub owner_email: String,
    pub url: String,
    pub location: String,
}
