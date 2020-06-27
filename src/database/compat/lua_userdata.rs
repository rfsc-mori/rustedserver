use crate::database::compat::DatabaseCompat;
use crate::database::compat::query_params::QueryParams;
use crate::error::from_lua_err;
use crate::parallel_tasks::TaskSender;
use crate::scripting::context::ScriptContextPool;
use crate::scripting::traits::ScriptRegistryEntry;
use anyhow::format_err;
use mlua::{ExternalError, UserData, UserDataMethods, Function, MultiValue, Variadic};
use tokio::task;

impl UserData for DatabaseCompat {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_async_method(
            "query",
            |_, db, (query, params): (mlua::String, Variadic<QueryParams>)| async move {
                Ok(db.query(query.to_str()?, params.into_iter()).await)
            }
        );

        methods.add_async_method(
            "rawQuery",
            |_, db, query: mlua::String| async move {
                Ok(db.raw_query(query.to_str()?).await)
            }
        );

        methods.add_async_method(
            "asyncQuery",
            |lua, db, (callback, query, params): (Option<String>, String, Variadic<QueryParams>)| async move {
                let task_sender = TaskSender::from_registry(lua)?;

                let task = {
                    let script_pool = ScriptContextPool::from_registry(lua)?;

                    task::spawn(async move {
                        let query_ok = db.query(query.as_str(), params.into_iter()).await;

                        if let Some(callback) = callback {
                            script_pool.with(move |lua| async move {
                                let lua = lua.lock().await;

                                let func = lua.globals()
                                    .get::<_, Function>(callback.as_str());

                                let result = match func {
                                    Ok(f) => f.call_async::<_, MultiValue>(query_ok).await,
                                    Err(e) => Err(e)
                                };

                                let result = result.map_err(from_lua_err);

                                log_errors!(result);

                                Ok(())
                            }).await?;
                        }

                        Ok(())
                    })
                };

                task_sender.send(task)
                    .map_err(ExternalError::to_lua_err)?;

                Ok(())
            }
        );

        methods.add_async_method(
            "storeQuery",
            |lua, db, (query, params): (String, Variadic<QueryParams>)| async move {
                let result = db.store_query(query, params.into_iter()).await;

                let value = if result.has_next().await {
                    lua.pack(result)?
                } else {
                    lua.pack(false)?
                };

                Ok(value)
            }
        );

        methods.add_async_method(
            "asyncStoreQuery",
            |lua, db, (callback, query, params): (Option<String>, String, Variadic<QueryParams>)| async move {
                let task_sender = TaskSender::from_registry(lua)?;

                let task = {
                    let script_pool = ScriptContextPool::from_registry(lua)?;

                    task::spawn(async move {
                        let result = db.store_query(query, params.into_iter()).await;

                        if let Some(callback) = callback {
                            script_pool.with(move |lua| async move {
                                let lua = lua.lock().await;

                                let func = lua.globals()
                                    .get::<_, Function>(callback.as_str());

                                let result = match func {
                                    Ok(f) => {
                                        let value = if result.has_next().await {
                                            lua.pack(result)?
                                        } else {
                                            lua.pack(false)?
                                        };

                                        f.call_async::<_, MultiValue>(value).await
                                    },
                                    Err(e) => Err(e)
                                };

                                let result = result.map_err(from_lua_err);

                                log_errors!(result);

                                Ok(())
                            }).await?;
                        }

                        Ok(())
                    })
                };

                task_sender.send(task)
                    .map_err(ExternalError::to_lua_err)?;

                Ok(())
            }
        );

        methods.add_async_method(
            "tableExists",
            |_, db, table: mlua::String| async move {
                Ok(db.table_exists(table.to_str()?).await)
            }
        );
    }
}
