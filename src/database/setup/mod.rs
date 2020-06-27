use crate::error::log_error;
use crate::database::handle::DatabaseHandle;
use crate::scripting::context::ScriptContextPool;
use anyhow::{format_err, Context, Result};
use std::path::PathBuf;
use tokio::fs;
use tracing::{debug, info, trace};
use mlua::Function;

pub struct DatabaseSetup {
    handle: DatabaseHandle,
    script_pool: ScriptContextPool,
}

impl DatabaseSetup {
    pub fn new(handle: DatabaseHandle, script_pool: ScriptContextPool) -> Self {
        Self {
            handle,
            script_pool,
        }
    }

    pub async fn validate_database(&self) -> Result<()> {
        self.validate_database_non_empty().await?;

        Ok(())
    }

    pub async fn update_database(&self) -> Result<()> {
        debug!("Running database migration scripts...");

        self.run_db_migration_scripts()
            .await
            .context("Failed to run database migration scripts.")?;

        Ok(())
    }

    async fn validate_database_non_empty(&self) -> Result<()> {
        debug!("Querying database table count...");

        let db_info = sqlx::query!(r#"
                SELECT
                    COUNT(`table_name`) AS count
                FROM
                    `information_schema`.`tables`
                WHERE
                    `table_schema` = DATABASE();
            "#)
            .fetch_one(self.handle.pool())
            .await
            .context("Failed to query database table count.")?;

        match db_info.count {
            count if count > 0 => Ok(()),
            _ => Err(format_err!("The database you have specified in the configuration is empty, \
                                  please import the `tfs/schema.sql` to your database."))
        }
    }

    async fn run_db_migration_scripts(&self) -> Result<()> {
        debug!("Querying database version...");

        let mut version = self.db_version()
            .await
            .context("Failed to query database version.")?;

        debug!("Database version: `{}`.", version);

        let migrations_path = PathBuf::from("./tfs/data/migrations/");

        loop {
            let mut script_path = migrations_path.clone();
            script_path.push(version.to_string());
            script_path.set_extension("lua");

            let updated = self.script_pool.with(move |lua| async move {
                let lua = lua.lock().await;

                let script_file = script_path.to_str()
                    .with_context(|| format!("Invalid filename: `{:?}`.", script_path))?;

                trace!("Running migration script: `{}`...", script_file);

                let script_source = fs::read(script_file)
                    .await
                    .with_context(|| format!("Failed to load script: `{}`.", script_file))?;

                lua.load(script_source.as_slice())
                    .set_name(script_file)?
                    .exec_async()
                    .await
                    .with_context(|| format!("Failed to run script: `{}`.", script_file))?;

                let migration: Function = lua.globals()
                    .get("onUpdateDatabase")
                    .with_context(|| {
                        format!("`{}` does not contain an `onUpdateDatabase` function.", script_file)
                    })?;

                let updated: bool = migration.call_async(())
                    .await
                    .with_context(|| {
                        format!("Failed to call `onUpdateDatabase` from `{}`.", script_file)
                    })?;

                Ok(updated)
            }).await?;

            if updated {
                version = version.checked_add(1)
                    .context("Database version number overflow.")?;

                debug!("Updating database version number...");

                self.set_db_version(version)
                    .await
                    .context("Failed to update database version number.")?;

                info!("Database has been updated to version `{}`.", version);
            } else {
                debug!("Nothing to update.");

                break
            }
        }

        Ok(())
    }

    async fn db_version(&self) -> Result<u64> {
        let db_config = sqlx::query!(r#"
                SELECT
                    CAST(`value` AS UNSIGNED) AS version
                FROM
                    `server_config`
                WHERE
                    `config` = "db_version";
            "#)
            .fetch_one(self.handle.pool())
            .await?;

        Ok(db_config.version)
    }

    async fn set_db_version(&self, version: u64) -> Result<()> {
        sqlx::query!(r#"
                UPDATE
                    `server_config`
                SET
                    `value` = ?
                WHERE
                    `config` = "db_version";
            "#, version)
            .execute(self.handle.pool())
            .await?;

        Ok(())
    }
}
