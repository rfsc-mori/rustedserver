use crate::settings::SQLOptions;
use anyhow::{format_err, Context, Result};
use sqlx::mysql::{MySqlConnectOptions, MySqlPool};
use tracing::{debug, info};

#[derive(Clone)]
pub struct DatabaseHandle {
    pool: MySqlPool
}

impl DatabaseHandle {
    pub async fn connect(options: &SQLOptions) -> Result<Self> {
        info!("Establishing database connection...");

        let mysql_opts = MySqlConnectOptions::new()
            .username(options.mysql_user.as_str())
            .password(options.mysql_pass.as_str())
            .host(options.mysql_host.as_str())
            .port(options.mysql_port)
            .database(options.mysql_db.as_str());

        let pool = MySqlPool::builder()
            .max_size(options.mysql_conn)
            .build_with(mysql_opts)
            .await
            .context("Failed to initialize the database connection pool.")?;

        // TODO: maxPacketSize stuff

        debug!("Testing database connection...");

        let db_info = sqlx::query!(r#"SELECT DATABASE() AS name;"#)
            .fetch_one(&pool)
            .await
            .context("Failed to query database name.")?;

        match db_info.name {
            Some(db_name) => {
                debug!("Connected to the database: `{}`.", db_name);

                let db = Self {
                    pool
                };

                Ok(db)
            },
            None => Err(format_err!("No database selected."))
        }
    }

    pub fn pool(&self) -> &MySqlPool {
        &self.pool
    }
}
