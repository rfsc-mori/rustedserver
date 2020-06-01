use serde::{Serialize, Deserialize};

#[derive(Default, Serialize, Deserialize)]
pub struct SQL {
    pub mysql_host: String,
    pub mysql_user: String,
    pub mysql_pass: String,
    pub mysql_db: String,
    pub mysql_port: i64,
    pub mysql_sock: String,
}
