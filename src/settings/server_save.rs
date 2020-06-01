use serde::{Serialize, Deserialize};

#[derive(Default, Serialize, Deserialize)]
pub struct ServerSave {
    pub server_save_notify_message: bool,
    pub server_save_notify_duration: i64,
    pub server_save_clean_map: bool,
    pub server_save_close: bool,
    pub server_save_shutdown: bool,
}
