use super::validation_error;
use validator::ValidationError;

pub type TDespawnRange = u32;

pub fn validate_despawn_range(despawn_range: &TDespawnRange) -> Result<(), ValidationError> {
    match *despawn_range {
        despawn_range if despawn_range > 0 => Ok(()),
        0 => Ok(()),
        _ => Err(validation_error("invalid_range",
                                  "Despawn range allowed range: `>= 0`."))
    }
}
