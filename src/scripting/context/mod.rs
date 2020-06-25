mod lua_userdata;

use crate::database::handle::DatabaseHandle;
use crate::parallel_tasks::TaskSender;
use crate::scripting::{debug, globals};
use crate::scripting::traits::ScriptRegistryEntry;
use anyhow::Result;
use deadpool::unmanaged::{Object, Pool};
use futures::Future;
use mlua::Lua;
use std::sync::Arc;
use tokio::task;
use tokio::sync::{Mutex, MutexGuard};

pub struct LuaHandle {
    lua: Arc<Mutex<Lua>>
}

impl LuaHandle {
    pub async fn lock(&self) -> MutexGuard<'_, Lua> {
        self.lua.lock().await
    }

    fn new(lua: Lua) -> Self {
        Self {
            lua: Arc::new(Mutex::new(lua))
        }
    }

    fn private_clone(&self) -> Self {
        Self {
            lua: Arc::clone(&self.lua)
        }
    }
}

pub struct ScriptContext {
    handle: LuaHandle
}

impl ScriptContext {
    pub fn new() -> Result<Self> {
        let lua = Lua::new_with(mlua::StdLib::ALL_SAFE)?;

        debug::log_lua_version(&lua)?;

        globals::bit::maybe_register_bit(&lua)?;

        let context = Self {
            handle: LuaHandle::new(lua)
        };

        Ok(context)
    }

    pub async fn register_db(&self, database: DatabaseHandle) -> Result<()> {
        let lua = self.handle.lock().await;
        globals::db::register_db(&lua, database)?;

        Ok(())
    }

    pub async fn register_task_sender(&self, task_tx: TaskSender) -> Result<()> {
        let lua = self.handle.lock().await;
        task_tx.add_to_registry(&lua)?;

        Ok(())
    }

    pub async fn unregister_task_sender(&self) -> Result<()> {
        let lua = self.handle.lock().await;
        TaskSender::remove_from_registry(&lua)?;

        Ok(())
    }

    async fn with<F, R, FR>(&self, func: F) -> Result<R>
    where
        F: 'static + Send + FnOnce(LuaHandle) -> FR,
        R: 'static + Send,
        FR: Future<Output = Result<R>>,
    {
        let lua = self.handle.private_clone();

        task::spawn_blocking(move || {
            let runtime = tokio::runtime::Handle::current();

            runtime.block_on(async move {
                func(lua).await
            })
        }).await?
    }
}

#[derive(Clone)]
pub struct ScriptContextPool {
    pool: Pool<ScriptContext>
}

impl ScriptContextPool {
    pub fn new(max_size: usize) -> Self {
        Self {
            pool: Pool::new(max_size)
        }
    }

    pub async fn add(&self, context: ScriptContext) -> Result<()> {
        self.clone().add_to_registry(&*context.handle.lua.lock().await)?;
        self.pool.add(context).await;

        Ok(())
    }

    // TODO: Warn that may cause a deadlock if used inside a .with call.
    pub async fn lock_all(&self) -> Vec<Object<ScriptContext>> {
        let mut contexts = Vec::with_capacity(self.pool.status().size);

        while contexts.len() < self.pool.status().size {
            contexts.push(self.pool.get().await);
        }

        contexts
    }

    #[allow(clippy::panic)] // Panic to prevent deadlock.
    pub async fn with<F, R, FR>(&self, func: F) -> Result<R>
    where
        F: 'static + Send + FnOnce(LuaHandle) -> FR,
        R: 'static + Send,
        FR: Future<Output = Result<R>>,
    {
        assert!(self.pool.status().size > 0, "No script VMs registered. Maybe stop() was called?");
        self.pool.get().await.with(func).await
    }

    // TODO: Warn that not calling may leak memory due to ref cycle.
    pub async fn close(self) -> Result<()> {
        while self.pool.status().size > 0 {
            self.pool.remove().await;
        }

        Ok(())
    }
}
