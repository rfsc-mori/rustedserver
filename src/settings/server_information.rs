use serde::{Serialize, Deserialize};

#[derive(Default, Serialize, Deserialize)]
pub struct ServerInformation {
    pub owner_name: String,
    pub owner_email: String,
    pub url: String,
    pub location: String,
}
