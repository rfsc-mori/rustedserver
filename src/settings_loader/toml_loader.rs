use crate::settings::Settings;
use super::error::SettingsLoaderError;
use super::error::Result;

pub async fn load<T: AsRef<[u8]>>(contents: T) -> Result<Settings> {
    let settings = toml::from_slice(contents.as_ref())
        .map_err(|e| SettingsLoaderError::DeserializeError(e.to_string()))?;

    Ok(settings)
}
