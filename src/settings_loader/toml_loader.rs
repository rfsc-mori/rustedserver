use crate::settings::Settings;
use super::error::SettingsLoaderError;
use super::error::Result;

pub async fn load<T: AsRef<[u8]>>(contents: T) -> Result<Settings> {
    match toml::from_slice(contents.as_ref()) {
        Ok(settings) => Ok(settings),
        Err(e) => Err(SettingsLoaderError::DeserializeError(e.to_string()))
    }
}
