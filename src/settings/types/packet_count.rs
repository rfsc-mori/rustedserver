use super::validation_error;
use validator::ValidationError;

pub type TPacketCount = u32;

#[allow(unused_comparisons)] // `>= 0` covers u32 range.
pub fn validate_packet_count(packet_count: &TPacketCount) -> Result<(), ValidationError> {
    match *packet_count {
        count if count >= 0 => Ok(()),
        _ => Err(validation_error("invalid_range",
                                  "Packet count allowed range: `>= 0`."))
    }
}
