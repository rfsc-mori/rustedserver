use anyhow::Result;
use mlua::{Lua, FromLua, ToLua};
use std::marker::Sized;

pub trait ScriptRegistryEntry {
    fn add_to_registry<'lua>(self, lua: &'lua Lua) -> Result<()>
    where
        Self: Clone + Sized + ToLua<'lua>
    {
        lua.set_named_registry_value(Self::lua_id(), self)?;

        Ok(())
    }

    fn from_registry<'lua>(lua: &'lua Lua) -> mlua::Result<Self>
    where
        Self: FromLua<'lua>
    {
        lua.named_registry_value(Self::lua_id())
    }

    fn remove_from_registry(lua: &Lua) -> Result<()> {
        lua.unset_named_registry_value(Self::lua_id())?;
        lua.gc_collect()?;

        Ok(())
    }

    fn lua_id() -> &'static str;
}
