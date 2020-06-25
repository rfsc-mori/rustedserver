#![allow(clippy::struct_excessive_bools)] // Config flags.
#![allow(clippy::wildcard_imports)] // Readability.

// TODO: Move some types to global scope.
mod types;

mod combat;
mod connection;
mod death;
mod housing;
mod item_usage;
mod map;
mod market;
mod sql;
mod misc;
mod server_save;
mod rates;
mod monsters;
mod stamina;
mod scripts;
mod startup;
mod server_information;

pub use combat::CombatOptions;
pub use connection::ConnectionOptions;
pub use death::DeathOptions;
pub use housing::HousingOptions;
pub use item_usage::ItemUsageOptions;
pub use map::MapOptions;
pub use market::MarketOptions;
pub use sql::SQLOptions;
pub use misc::MiscOptions;
pub use server_save::ServerSaveOptions;
pub use rates::RateOptions;
pub use monsters::MonsterOptions;
pub use stamina::StaminaOptions;
pub use scripts::ScriptOptions;
pub use startup::StartupOptions;
pub use server_information::ServerInformationOptions;

use serde::{Serialize, Deserialize};
use validator::Validate;
use validator_derive::Validate;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct Settings {
    #[validate]
    pub combat: CombatOptions,
    #[validate]
    pub connection: ConnectionOptions,
    #[validate]
    pub death: DeathOptions,
    #[validate]
    pub housing: HousingOptions,
    #[validate]
    pub item_usage: ItemUsageOptions,
    #[validate]
    pub map: MapOptions,
    #[validate]
    pub market: MarketOptions,
    #[validate]
    pub sql: SQLOptions,
    #[validate]
    pub misc: MiscOptions,
    #[validate]
    pub server_save: ServerSaveOptions,
    #[validate]
    pub rates: RateOptions,
    #[validate]
    pub monsters: MonsterOptions,
    #[validate]
    pub stamina: StaminaOptions,
    #[validate]
    pub scripts: ScriptOptions,
    #[validate]
    pub startup: StartupOptions,
    #[validate]
    pub server_information: ServerInformationOptions,
}

use anyhow::{ensure, format_err, Context, Result};
use config::{Config, File, FileFormat};
use itertools::Itertools;
use std::path::PathBuf;
use tokio::task;
use tracing::debug;

impl Settings {
    pub async fn load_from_dir(path: PathBuf) -> Result<Self> {
        task::spawn_blocking(|| load_from_dir(path)).await?
    }
}

fn load_from_dir(mut dir: PathBuf) -> Result<Settings> {
    ensure!(dir.is_dir(), "Invalid directory: `{:?}`.", dir);

    dir.push("*.toml");

    let path_glob = dir.to_str()
        .with_context(|| format!("Invalid directory path: `{:?}`.", dir))?;

    let mut config = Config::new();

    let files: Vec<_> = glob::glob(path_glob)?
        .map(|result| {
            result
                .map_err(|e| format_err!(e))
                .and_then(|path| {
                    match path.to_str() {
                        Some(file) => Ok(file.to_owned()),
                        None => Err(format_err!("Invalid filename: `{:?}`.", path))
                    }
                })
        })
        .try_collect()?;

    for file in files {
        debug!("Merging config from: `{}`...", file);
        config.merge(File::new(file.as_str(), FileFormat::Toml))?;
    }

    debug!("Deserializing settings...");
    let settings: Settings = config.try_into()
        .context("Failed to deserialize settings.")?;

    // TODO: Use validated_newtype
    debug!("Validating settings...");
    settings.validate()
        .context("Invalid settings loaded.")?;

    Ok(settings)
}
