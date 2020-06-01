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
    combat: Combat,
    connection: Connection,
    death: Death,
    housing: Housing,
    item_usage: ItemUsage,
    map: Map,
    market: Market,
    sql: SQL,
    misc: Misc,
    server_save: ServerSave,
    rates: Rates,
    monsters: Monsters,
    stamina: Stamina,
    scripts: Scripts,
    startup: Startup,
    server_information: ServerInformation,
}

impl Settings {
    pub fn new() -> Self {
        Self::default()
    }
}
