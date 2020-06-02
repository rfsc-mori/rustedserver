mod error;
mod toml_loader;

use crate::settings::Settings;
use error::SettingsLoaderError;
use error::Result;

use std::path::Path;
use tokio::fs;
use tracing::info;
use validator::Validate;

async fn get_toml_config_file() -> Result<&'static str> {
    const TOML_DIST: &str = "./config.toml.dist";
    const TOML_FILE: &str = "./config.toml";

    if !Path::new(TOML_FILE).exists() {
        if !Path::new(TOML_DIST).exists() {
            return Err(SettingsLoaderError::ConfigDistMissing(String::from(TOML_DIST)));
        }

        info!("Copying `{0}` to `{1}`.", TOML_DIST, TOML_FILE);
        fs::copy(TOML_DIST, TOML_FILE).await?;
    }

    Ok(TOML_FILE)
}

async fn load_settings_toml() -> Result<Settings> {
    let toml_file = get_toml_config_file().await?;
    let contents = fs::read(toml_file).await?;
    let settings = toml_loader::load(contents).await?;

    Ok(settings)
}

async fn validate_settings(settings: &Settings) -> Result<()> {
    match settings.validate() {
        Ok(_) => Ok(()),
        Err(e) => Err(SettingsLoaderError::ValidationError(e.to_string()))
    }
}

pub async fn load_settings() -> Result<Settings> {
    info!("Loading config.");
    let settings = load_settings_toml().await?;
    validate_settings(&settings).await?;

    Ok(settings)
}
