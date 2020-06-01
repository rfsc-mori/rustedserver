use serde::{Serialize, Deserialize};

#[derive(Default, Serialize, Deserialize)]
pub struct Stamina {
    pub stamina_system: bool,
}
