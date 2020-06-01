use serde::{Serialize, Deserialize};

#[derive(Default, Serialize, Deserialize)]
pub struct Combat {
    world_type: String,
    hotkey_aimbot_enabled: bool,
    protection_level: i64,
    kills_to_red_skull: i64,
    kills_to_black_skull: i64,
    pz_locked: i64,
    remove_charges_from_runes: bool,
    remove_charges_from_potions: bool,
    remove_weapon_ammunition: bool,
    remove_weapon_charges: bool,
    time_to_decrease_frags: i64,
    white_skull_time: i64,
    stair_jump_exhaustion: i64,
    experience_by_killing_players: bool,
    exp_from_players_level_range: i64,
}
