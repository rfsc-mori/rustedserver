use serde::{Serialize, Deserialize};

#[derive(Default, Serialize, Deserialize)]
pub struct Scripts {
    pub warn_unsafe_scripts: bool,
    pub convert_unsafe_scripts: bool,
}
