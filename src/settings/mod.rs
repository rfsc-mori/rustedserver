mod types;

pub mod combat;
pub mod connection;
pub mod death;
pub mod housing;
pub mod item_usage;
pub mod map;
pub mod market;
pub mod sql;
pub mod misc;
pub mod server_save;
pub mod rates;
pub mod monsters;
pub mod stamina;
pub mod scripts;
pub mod startup;
pub mod server_information;

use combat::CombatOptions;
use connection::ConnectionOptions;
use death::DeathOptions;
use housing::HousingOptions;
use item_usage::ItemUsageOptions;
use map::MapOptions;
use market::MarketOptions;
use sql::SQLOptions;
use misc::MiscOptions;
use server_save::ServerSaveOptions;
use rates::RateOptions;
use monsters::MonsterOptions;
use stamina::StaminaOptions;
use scripts::ScriptOptions;
use startup::StartupOptions;
use server_information::ServerInformationOptions;

use serde::{Serialize, Deserialize};
use validator::Validate;
use validator_derive::Validate;

#[derive(Debug, Default, Serialize, Deserialize, Validate)]
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
