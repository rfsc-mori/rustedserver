use anyhow::Result;
use mlua::{Lua, Value};
use tracing::debug;

pub fn log_lua_version(lua: &Lua) -> Result<()> {
    let lua_ver = {
        match lua.globals().get("_VERSION")? {
            Value::String(ver) => ver.to_str()?.to_owned(),
            _ => "unknown".to_owned()
        }
    };

    let jit_ver = {
        match lua.globals().get("jit")? {
            Value::Table(jit) => {
                match jit.get("version")? {
                    Value::String(ver) => ver.to_str()?.to_owned(),
                    _ => "N/A".to_owned()
                }
            },
            _ => "N/A".to_owned()
        }
    };

    debug!("Script VM version: `{0}` - `{1}`", lua_ver, jit_ver);

    Ok(())
}
