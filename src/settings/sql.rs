use super::types::*;
use serde::{Serialize, Deserialize};
use validator::Validate;
use validator_derive::Validate;

#[derive(Debug, Default, Serialize, Deserialize, Validate)]
pub struct SQL {
    #[validate(custom = "validate_host_address")]
    pub mysql_host: THostAddress,
    pub mysql_user: String,
    pub mysql_pass: String,
    #[validate(custom = "validate_database_name")]
    pub mysql_db: TDatabaseName,
    #[validate(custom = "validate_network_port")]
    pub mysql_port: TNetworkPort,
    pub mysql_sock: String,
}
