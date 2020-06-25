use crate::scripting::traits::ScriptRegistryEntry;
use mlua::UserData;

impl ScriptRegistryEntry for super::TaskSender {
    fn lua_id() -> &'static str {
        "task_sender"
    }
}

impl UserData for super::TaskSender {}
