use serde::{Serialize, Deserialize};

#[derive(Default, Serialize, Deserialize)]
pub struct ServerSave {
    server_save_notify_message: bool,
    server_save_notify_duration: i64,
    server_save_clean_map: bool,
    server_save_close: bool,
    server_save_shutdown: bool,
}
