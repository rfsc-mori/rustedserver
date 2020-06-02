use super::validation_error;
use validator::ValidationError;

pub type TMessageQueuedCount = u32;

pub fn validate_message_queued_count(message_queued_count: &TMessageQueuedCount)
    -> Result<(), ValidationError> {
    match *message_queued_count {
        message_queued_count if message_queued_count > 0 => Ok(()),
        0 => Ok(()),
        _ => Err(validation_error("invalid_range",
                                  "Message queued count allowed range: `>= 0`."))
    }
}
