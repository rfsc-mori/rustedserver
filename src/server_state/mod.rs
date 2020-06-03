use crate::database::Database;
use crate::settings::Settings;
use anyhow::{Context, Result};
use tokio::sync::RwLock;
use tokio::sync::RwLockReadGuard;
use std::sync::Arc;

pub struct ServerState {
    settings: RwLock<Settings>,
    database: Database,
}

impl ServerState {
    pub async fn settings(&self) -> RwLockReadGuard<'_, Settings> {
        self.settings.read().await
    }
}

pub async fn init_state(settings: Settings) -> Result<Arc<ServerState>> {
    let database = Database::new(&settings.sql)
        .await
        .context("Failed to initialize the database system.")?;

    if !database.schema().is_database_setup().await.context("Failed to check database state.")? {
        return Err(anyhow::anyhow!("The database you have specified in config.lua is empty, \
                                    please import the schema.sql to your database."));
    }

    let server = Arc::new(ServerState {
        settings: RwLock::new(settings),
        database
    });

    Ok(server)
}
