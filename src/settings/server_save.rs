use super::types::*;
use serde::{Serialize, Deserialize};
use validator::Validate;
use validator_derive::Validate;

#[derive(Debug, Default, Serialize, Deserialize, Validate)]
pub struct ServerSaveOptions {
    pub server_save_notify_message: bool,
    #[validate(custom = "validate_shutdown_notify_minutes")]
    pub server_save_notify_duration: TShutdownNotifyMinutes,
    pub server_save_clean_map: bool,
    pub server_save_close: bool,
    pub server_save_shutdown: bool,
}
