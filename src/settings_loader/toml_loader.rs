use crate::settings::Settings;
use anyhow::{Context, Result};

pub async fn load<T: AsRef<[u8]>>(contents: T) -> Result<Settings> {
    let settings = toml::from_slice(contents.as_ref())
        .context("Failed to deserialize settings.")?;

    Ok(settings)
}
