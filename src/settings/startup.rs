use serde::{Serialize, Deserialize};

#[derive(Default, Serialize, Deserialize)]
pub struct Startup {
    default_priority: String,
    startup_database_optimization: bool,
}
