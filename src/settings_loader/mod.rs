mod error;
mod toml_loader;

use crate::settings::Settings;
use error::SettingsLoaderError;
use error::Result;

use std::path::Path;
use tokio::fs;

async fn get_toml_config_file() -> Result<&'static str> {
    const TOML_DIST: &str = "./config.toml.dist";
    const TOML_FILE: &str = "./config.toml";

    if !Path::new(TOML_FILE).exists() {
        if !Path::new(TOML_DIST).exists() {
            return Err(SettingsLoaderError::ConfigDistMissing(String::from(TOML_DIST)));
        }

        fs::copy(TOML_DIST, TOML_FILE).await?;
    }

    Ok(TOML_FILE)
}

async fn load_config_toml() -> Result<Settings> {
    let toml_file = get_toml_config_file().await?;
    let contents = fs::read(toml_file);

    toml_loader::load(contents.await?).await
}

pub async fn load_config() -> Result<Settings> {
    load_config_toml().await
}
