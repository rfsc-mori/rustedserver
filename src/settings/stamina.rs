use serde::{Serialize, Deserialize};

#[derive(Default, Serialize, Deserialize)]
pub struct Stamina {
    stamina_system: bool,
}
