use crate::settings::Settings;
use anyhow::{Context, Result};
use tokio::sync::RwLock;
use tokio::sync::RwLockReadGuard;
use std::sync::Arc;

pub struct ServerState {
    settings: RwLock<Settings>,
}

impl ServerState {
    pub async fn settings(&self) -> RwLockReadGuard<'_, Settings> {
        self.settings.read().await
    }
}

pub async fn init_state(settings: Settings) -> Result<Arc<ServerState>> {
    let server = Arc::new(ServerState {
        settings: RwLock::new(settings)
    });

    Ok(server)
}
