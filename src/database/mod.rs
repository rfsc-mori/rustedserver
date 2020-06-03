mod connection_string;

pub mod schema;

use crate::settings::sql::SQLOptions;
use anyhow::{Context, Result};
use connection_string::ConnectionStringBuilder;
use schema::DatabaseSchema;
use sqlx::mysql::MySqlPool;
use tracing::info;

pub struct Database {
    pool: MySqlPool,
}

impl Database {
    pub async fn new(options: &SQLOptions) -> Result<Database> {
        let conn_url = ConnectionStringBuilder::new()
            .scheme("mysql".to_owned())
            .user(options.mysql_user.clone())
            .password(options.mysql_pass.clone())
            .host(options.mysql_host.clone())
            .port(options.mysql_port.clone())
            .database(options.mysql_db.clone())
            .build()
            .context("Failed to build the connection string.")?;

        info!("Establishing database connection...");
        let pool = MySqlPool::builder()
            .max_size(options.mysql_conn)
            .build(conn_url.as_str())
            .await
            .context("Failed to initialize the database connection pools.")?;

        let db = Database { pool };

        let db_info = sqlx::query!("select database() as name;")
            .fetch_one(&db.pool)
            .await
            .context("Failed to query the database name.")?;

        if let Some(db_name) = &db_info.name {
            tracing::info!("Connected to the database: `{}`.", db_name);
        }

        Ok(db)
    }

    pub fn schema(&self) -> DatabaseSchema {
        DatabaseSchema::new(&self.pool)
    }
}
