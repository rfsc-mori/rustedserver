mod toml_loader;

use crate::settings::Settings;
use anyhow::{Context, Result};
use std::path::Path;
use tokio::fs;
use tracing::info;
use validator::Validate;

async fn find_toml_config_file() -> Result<&'static str> {
    const TOML_DIST: &str = "./config.toml.dist";
    const TOML_FILE: &str = "./config.toml";

    if !Path::new(TOML_FILE).exists() {
        if !Path::new(TOML_DIST).exists() {
            return Err(anyhow::anyhow!("Could not find the distribution config file: `{}`.",
                                       TOML_DIST));
        }

        info!("Copying `{0}` to `{1}`.", TOML_DIST, TOML_FILE);
        fs::copy(TOML_DIST, TOML_FILE)
            .await
            .with_context(|| {
                format!("Failed to copy the distribution config file from `{}` to `{}`.",
                        TOML_DIST,
                        TOML_FILE)
            })?;
    }

    Ok(TOML_FILE)
}

async fn load_settings_toml() -> Result<Settings> {
    let toml_file = find_toml_config_file()
        .await
        .context("Failed to find the `.toml` config file.")?;

    let contents = fs::read(toml_file)
        .await
        .with_context(|| {
            format!("Failed to read the file: `{}`.", toml_file)
        })?;

    let settings = toml_loader::load(contents)
        .await
        .with_context(|| {
            format!("Failed to load settings from: `{}`.", toml_file)
        })?;

    Ok(settings)
}

fn validate_settings(settings: &Settings) -> Result<()> {
    Ok(settings.validate()?)
}

pub async fn load_settings() -> Result<Settings> {
    info!("Loading config.");

    let settings = load_settings_toml()
        .await
        .context("Failed to load settings from the `.toml` file.")?;

    validate_settings(&settings)
        .context("Failed to validate settings.")?;

    Ok(settings)
}
