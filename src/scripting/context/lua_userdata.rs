use crate::scripting::traits::ScriptRegistryEntry;
use mlua::UserData;

impl ScriptRegistryEntry for super::ScriptContextPool {
    fn lua_id() -> &'static str {
        "vm_pool"
    }
}

impl UserData for super::ScriptContextPool {}
