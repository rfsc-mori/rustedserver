use serde::{Serialize, Deserialize};

#[derive(Default, Serialize, Deserialize)]
pub struct Combat {
    pub world_type: String,
    pub hotkey_aimbot_enabled: bool,
    pub protection_level: i64,
    pub kills_to_red_skull: i64,
    pub kills_to_black_skull: i64,
    pub pz_locked: i64,
    pub remove_charges_from_runes: bool,
    pub remove_charges_from_potions: bool,
    pub remove_weapon_ammunition: bool,
    pub remove_weapon_charges: bool,
    pub time_to_decrease_frags: i64,
    pub white_skull_time: i64,
    pub stair_jump_exhaustion: i64,
    pub experience_by_killing_players: bool,
    pub exp_from_players_level_range: i64,
}
