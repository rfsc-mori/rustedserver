use serde::{Serialize, Deserialize};

#[derive(Default, Serialize, Deserialize)]
pub struct Misc {
    pub allow_change_outfit: bool,
    pub free_premium: bool,
    pub kick_idle_player_after_minutes: i64,
    pub max_message_buffer: i64,
    pub emote_spells: bool,
    pub classic_equipment_slots: bool,
    pub classic_attack_speed: bool,
    pub show_scripts_log_in_console: bool,
    pub show_online_status_in_char_list: bool,
    pub yell_minimum_level: i64,
    pub yell_always_allow_premium: bool,
    pub force_monster_type_on_load: bool,
}
