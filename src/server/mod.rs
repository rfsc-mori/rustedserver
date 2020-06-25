use crate::database::handle::DatabaseHandle;
use crate::database::setup::DatabaseSetup;
use crate::error;
use crate::error::log_error;
use crate::parallel_tasks;
use crate::parallel_tasks::TaskSender;
use crate::scripting::context::{ScriptContext, ScriptContextPool};
use crate::settings::Settings;
use anyhow::{Context, Result};
use tokio::{join, task};
use tracing::{debug, info};

pub async fn run(settings: Settings) -> Result<()> {
    debug!("Setting up error handler...");
    let (err_tx, err_rx) = error::create_error_channel();

    info!("Initializing task system...");
    let (task_tx, tasks_handle) = parallel_tasks::start(err_tx);

    info!("Initializing database system...");
    let database = DatabaseHandle::connect(&settings.sql)
        .await
        .context("Failed to initialize the database system.")?;

    let vm_count = match settings.scripts.script_vm_count {
        vm_count if vm_count > 0 => vm_count,
        0 => num_cpus::get(),
        _ => unreachable!("Reaching this means vm_count(usize) was negative.")
    };

    info!("Initializing script VM pool with `{}` instances...", vm_count);
    let script_pool = start_vm_pool(vm_count, database.clone(), task_tx)
        .await
        .context("Failed to initialize script VMs.")?;

    let server_handle = {
        let script_pool = script_pool.clone();

        task::spawn(async move {
            log_errors! {
                run_server(database, script_pool.clone()).await;
            }

            for context in script_pool.lock_all().await {
                log_errors! {
                    context.unregister_task_sender().await;
                }
            }
        })
    };

    let (server, tasks) = join!(server_handle, tasks_handle);
    log_errors!(server, tasks);

    // TODO: Maybe use an Arc<RwLock<Option<_>>> instead
    info!("Stopping script VMs...");
    script_pool.close().await?;

    Ok(())
}

async fn start_vm_pool(vm_count: usize,
                       database: DatabaseHandle,
                       task_tx: TaskSender) -> Result<ScriptContextPool> {
    let script_pool = ScriptContextPool::new(vm_count);

    for _ in 0..vm_count {
        let context = ScriptContext::new()?;

        context.register_db(database.clone()).await?;
        context.register_task_sender(task_tx.clone()).await?;

        script_pool.add(context).await?;
    }

    Ok(script_pool)
}

async fn run_server(database: DatabaseHandle, script_pool: ScriptContextPool) -> Result<()> {
    info!("Running startup tasks...");
    run_startup_tasks(database, script_pool)
        .await
        .context("Failed to run startup tasks.")?;

    Ok(())
}

async fn run_startup_tasks(database: DatabaseHandle, script_pool: ScriptContextPool) -> Result<()> {
    let setup = DatabaseSetup::new(database, script_pool);

    debug!("Validating database...");
    setup.validate_database()
        .await
        .context("Invalid database state.")?;

    debug!("Updating database...");
    setup.update_database()
        .await
        .context("Failed to update database.")?;

    Ok(())
}
