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

use combat::Combat;
use connection::Connection;
use death::Death;
use housing::Housing;
use item_usage::ItemUsage;
use map::Map;
use market::Market;
use sql::SQL;
use misc::Misc;
use server_save::ServerSave;
use rates::Rates;
use monsters::Monsters;
use stamina::Stamina;
use scripts::Scripts;
use startup::Startup;
use server_information::ServerInformation;

use serde::{Serialize, Deserialize};
use validator::Validate;
use validator_derive::Validate;

#[derive(Debug, Default, Serialize, Deserialize, Validate)]
pub struct Settings {
    #[validate]
    pub combat: Combat,
    #[validate]
    pub connection: Connection,
    #[validate]
    pub death: Death,
    #[validate]
    pub housing: Housing,
    #[validate]
    pub item_usage: ItemUsage,
    #[validate]
    pub map: Map,
    #[validate]
    pub market: Market,
    #[validate]
    pub sql: SQL,
    #[validate]
    pub misc: Misc,
    #[validate]
    pub server_save: ServerSave,
    #[validate]
    pub rates: Rates,
    #[validate]
    pub monsters: Monsters,
    #[validate]
    pub stamina: Stamina,
    #[validate]
    pub scripts: Scripts,
    #[validate]
    pub startup: Startup,
    #[validate]
    pub server_information: ServerInformation,
}
