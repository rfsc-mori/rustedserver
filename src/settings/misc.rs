use super::types::*;
use serde::{Serialize, Deserialize};
use validator::Validate;
use validator_derive::Validate;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct MiscOptions {
    pub allow_change_outfit: bool,
    pub free_premium: bool,
    #[validate(custom = "validate_idle_timeout")]
    pub kick_idle_player_after_minutes: TIdleTimeout,
    #[validate(custom = "validate_message_queued_count")]
    pub max_message_buffer: TMessageQueuedCount,
    pub emote_spells: bool,
    pub classic_equipment_slots: bool,
    pub classic_attack_speed: bool,
    pub show_scripts_log_in_console: bool,
    pub show_online_status_in_char_list: bool,
    #[validate(custom = "validate_level")]
    pub yell_minimum_level: TLevel,
    pub yell_always_allow_premium: bool,
    pub force_monster_type_on_load: bool,
    pub clean_protection_zones: bool,
}
