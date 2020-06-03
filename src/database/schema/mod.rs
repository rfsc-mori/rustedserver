use anyhow::{Context, Result};
use sqlx::mysql::MySqlPool;

pub struct DatabaseSchema<'a> {
    pool: &'a MySqlPool,
}

impl DatabaseSchema<'_> {
    pub fn new(pool: &MySqlPool) -> DatabaseSchema {
        DatabaseSchema { pool }
    }

    pub async fn is_database_setup(&self) -> Result<bool> {
        let db_info = sqlx::query!("
                select COUNT(`table_name`) as count
                from `information_schema`.`tables`
                where `table_schema` = database()
            ")
            .fetch_one(self.pool)
            .await
            .context("Failed to query the database table schema.")?;

        Ok(db_info.count > 0)
    }
}
