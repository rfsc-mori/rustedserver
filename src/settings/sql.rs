use serde::{Serialize, Deserialize};

#[derive(Default, Serialize, Deserialize)]
pub struct SQL {
    mysql_host: String,
    mysql_user: String,
    mysql_pass: String,
    mysql_db: String,
    mysql_port: i64,
    mysql_sock: String,
}
