use super::types::*;
use serde::{Serialize, Deserialize};
use validator::Validate;
use validator_derive::Validate;

#[derive(Debug, Default, Serialize, Deserialize, Validate)]
pub struct ConnectionOptions {
    #[validate(custom = "validate_ip_address")]
    pub ip: TIpAddress,
    pub bind_only_global_address: bool,
    #[validate(custom = "validate_network_port")]
    pub login_protocol_port: TNetworkPort,
    #[validate(custom = "validate_network_port")]
    pub game_protocol_port: TNetworkPort,
    #[validate(custom = "validate_network_port")]
    pub status_protocol_port: TNetworkPort,
    #[validate(custom = "validate_player_count")]
    pub max_players: TPlayerCount,
    pub motd: String,
    pub one_player_online_per_account: bool,
    pub allow_clones: bool,
    pub server_name: String,
    #[validate(custom = "validate_timeout_seconds")]
    pub status_timeout: TTimeoutSeconds,
    pub replace_kick_on_login: bool,
    #[validate(custom = "validate_packet_count")]
    pub max_packets_per_second: TPacketCount,
}
