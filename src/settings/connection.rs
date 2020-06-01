use serde::{Serialize, Deserialize};

#[derive(Default, Serialize, Deserialize)]
pub struct Connection {
    ip: String,
    bind_only_global_address: bool,
    login_protocol_port: i64,
    game_protocol_port: i64,
    status_protocol_port: i64,
    max_players: i64,
    motd: String,
    one_player_online_per_account: bool,
    allow_clones: bool,
    server_name: String,
    status_timeout: i64,
    replace_kick_on_login: bool,
    max_packets_per_second: i64,
}
