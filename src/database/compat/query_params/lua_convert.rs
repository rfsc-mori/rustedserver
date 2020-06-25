use crate::database::compat::query_params::QueryParams;
use anyhow::format_err;
use mlua::{ExternalError, Lua, FromLua, Value};

impl<'lua> FromLua<'lua> for QueryParams {
    fn from_lua(value: Value<'lua>, _lua: &'lua Lua) -> mlua::Result<Self> {
        match value {
            Value::Boolean(boolean) => Ok(Self::Boolean(boolean)),
            Value::Integer(integer) => Ok(Self::Integer(integer)),
            Value::Number(number) => Ok(Self::Number(number)),
            Value::String(lua_string) => Ok(Self::String(lua_string.as_bytes().into())),
            _ => Err(format_err!("Lua type `{:?}` cannot be converted to MySQL param.", value)
                     .to_lua_err())
        }
    }
}
