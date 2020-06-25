use super::validation_error;
use validator::ValidationError;

pub type TNetworkPort = u16;

#[allow(unreachable_patterns)] // `0..=65535` covers u16 range.
#[allow(clippy::decimal_literal_representation)] // Port number.
pub fn validate_network_port(network_port: &TNetworkPort) -> Result<(), ValidationError> {
    match *network_port {
        0..=65535 => Ok(()),
        _ => Err(validation_error("invalid_range",
                                  "Network port allowed range: `0..65535`."))
    }
}
