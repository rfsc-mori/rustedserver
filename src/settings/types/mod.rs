mod action_delay;
mod database_name;
mod death_penalty_value;
mod despawn_range;
mod host_address;
mod house_price;
mod house_rent_period;
mod idle_timeout;
mod ip_address;
mod kill_count;
mod level;
mod map_name;
mod message_queued_count;
mod network_port;
mod offer_count;
mod offer_duration;
mod offer_expired_interval;
mod packet_count;
mod player_count;
mod process_priority;
mod rate;
mod second_ticks;
mod shutdown_notify_minutes;
mod sql_connections;
mod timeout_seconds;
mod world_type;

pub use action_delay::*;
pub use database_name::*;
pub use death_penalty_value::*;
pub use despawn_range::*;
pub use host_address::*;
pub use house_price::*;
pub use house_rent_period::*;
pub use idle_timeout::*;
pub use ip_address::*;
pub use kill_count::*;
pub use level::*;
pub use map_name::*;
pub use message_queued_count::*;
pub use network_port::*;
pub use offer_count::*;
pub use offer_duration::*;
pub use offer_expired_interval::*;
pub use packet_count::*;
pub use player_count::*;
pub use process_priority::*;
pub use rate::*;
pub use second_ticks::*;
pub use shutdown_notify_minutes::*;
pub use sql_connections::*;
pub use timeout_seconds::*;
pub use world_type::*;

use std::borrow::Cow;
use std::collections::HashMap;
use validator::ValidationError;

fn validation_error(code: &'static str, message: &'static str) -> ValidationError {
    ValidationError {
        code: Cow::from(code),
        message: Some(Cow::from(message)),
        params: HashMap::new()
    }
}
