use crate::database::compat::query_result::QueryResult;
use mlua::{UserData, UserDataMethods};

impl UserData for QueryResult {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_async_method(
            "getNumber",
            |_, result, column: mlua::String| async move {
                Ok(result.get_number(column.to_str()?).await)
            }
        );

        methods.add_async_method(
            "getString",
            |_, result, column: mlua::String| async move {
                Ok(result.get_string(column.to_str()?).await)
            }
        );

        methods.add_async_method(
            "getStream",
            |lua, result, column: mlua::String| async move {
                let value = result.get_stream(column.to_str()?).await;
                Ok((lua.create_string(value.as_slice())?, value.len()))
            }
        );

        methods.add_async_method(
            "next",
            |_, result, ()| async move {
                Ok(result.fetch_next().await)
            }
        );

        methods.add_async_method(
            "free",
            |_, result, ()| async move {
                Ok(result.free().await)
            }
        );
    }
}
