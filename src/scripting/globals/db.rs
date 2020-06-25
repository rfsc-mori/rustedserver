use crate::database::compat::DatabaseCompat;
use crate::database::handle::DatabaseHandle;
use anyhow::Result;
use mlua::Lua;

pub fn register_db(lua: &Lua, database: DatabaseHandle) -> Result<()> {
    lua.globals().set("db", DatabaseCompat::new(database))?;

    Ok(())
}
