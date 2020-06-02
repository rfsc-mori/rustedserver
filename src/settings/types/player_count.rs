use super::validation_error;
use validator::ValidationError;

pub type TPlayerCount = u32;

pub fn validate_player_count(player_count: &TPlayerCount) -> Result<(), ValidationError> {
    match *player_count {
        player_count if player_count > 0 => Ok(()),
        0 => Ok(()),
        _ => Err(validation_error("invalid_range",
                                  "Player count allowed range: `>= 0`."))
    }
}
