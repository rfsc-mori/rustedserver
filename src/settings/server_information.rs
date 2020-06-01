use serde::{Serialize, Deserialize};

#[derive(Default, Serialize, Deserialize)]
pub struct ServerInformation {
    owner_name: String,
    owner_email: String,
    url: String,
    location: String,
}
