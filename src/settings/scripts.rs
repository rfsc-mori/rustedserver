use serde::{Serialize, Deserialize};

#[derive(Default, Serialize, Deserialize)]
pub struct Scripts {
    warn_unsafe_scripts: bool,
    convert_unsafe_scripts: bool,
}
