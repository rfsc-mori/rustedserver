use serde::{Serialize, Deserialize};

#[derive(Default, Serialize, Deserialize)]
pub struct Startup {
    pub default_priority: String,
    pub startup_database_optimization: bool,
}
