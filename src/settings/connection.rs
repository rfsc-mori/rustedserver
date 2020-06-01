use serde::{Serialize, Deserialize};

#[derive(Default, Serialize, Deserialize)]
pub struct Connection {
    pub ip: String,
    pub bind_only_global_address: bool,
    pub login_protocol_port: i64,
    pub game_protocol_port: i64,
    pub status_protocol_port: i64,
    pub max_players: i64,
    pub motd: String,
    pub one_player_online_per_account: bool,
    pub allow_clones: bool,
    pub server_name: String,
    pub status_timeout: i64,
    pub replace_kick_on_login: bool,
    pub max_packets_per_second: i64,
}
