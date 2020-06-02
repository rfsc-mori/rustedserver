use thiserror::Error;

#[derive(Error, Debug)]
pub enum SettingsLoaderError {
    #[error("The distributed settings file is missing: {0}")]
    ConfigDistMissing(String),
    #[error("Failed to deserialize configuration from: {0}")]
    DeserializeError(String),
    #[error(transparent)]
    ValidationError(#[from] validator::ValidationErrors),
    #[error(transparent)]
    IoError(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, SettingsLoaderError>;
