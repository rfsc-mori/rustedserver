use super::types::*;
use serde::{Serialize, Deserialize};
use validator::Validate;
use validator_derive::Validate;

#[derive(Debug, Default, Serialize, Deserialize, Validate)]
pub struct CombatOptions {
    #[validate(custom = "validate_world_type")]
    pub world_type: TWorldType,
    pub hotkey_aimbot_enabled: bool,
    #[validate(custom = "validate_level")]
    pub protection_level: TLevel,
    #[validate(custom = "validate_kill_count")]
    pub kills_to_red_skull: TKillCount,
    #[validate(custom = "validate_kill_count")]
    pub kills_to_black_skull: TKillCount,
    #[validate(custom = "validate_second_ticks")]
    pub pz_locked: TSecondTicks,
    pub remove_charges_from_runes: bool,
    pub remove_charges_from_potions: bool,
    pub remove_weapon_ammunition: bool,
    pub remove_weapon_charges: bool,
    #[validate(custom = "validate_second_ticks_special")]
    pub time_to_decrease_frags: TSecondTicks,
    #[validate(custom = "validate_second_ticks_special")]
    pub white_skull_time: TSecondTicks,
    #[validate(custom = "validate_second_ticks")]
    pub stair_jump_exhaustion: TSecondTicks,
    pub experience_by_killing_players: bool,
    #[validate(custom = "validate_level")]
    pub exp_from_players_level_range: TLevel,
}
