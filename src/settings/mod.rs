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

#[derive(Default, Serialize, Deserialize)]
pub struct Settings {
    pub combat: Combat,
    pub connection: Connection,
    pub death: Death,
    pub housing: Housing,
    pub item_usage: ItemUsage,
    pub map: Map,
    pub market: Market,
    pub sql: SQL,
    pub misc: Misc,
    pub server_save: ServerSave,
    pub rates: Rates,
    pub monsters: Monsters,
    pub stamina: Stamina,
    pub scripts: Scripts,
    pub startup: Startup,
    pub server_information: ServerInformation,
}
