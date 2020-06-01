use serde::{Serialize, Deserialize};

#[derive(Default, Serialize, Deserialize)]
pub struct Misc {
    allow_change_outfit: bool,
    free_premium: bool,
    kick_idle_player_after_minutes: i64,
    max_message_buffer: i64,
    emote_spells: bool,
    classic_equipment_slots: bool,
    classic_attack_speed: bool,
    show_scripts_log_in_console: bool,
    show_online_status_in_char_list: bool,
    yell_minimum_level: i64,
    yell_always_allow_premium: bool,
    force_monster_type_on_load: bool,
}
