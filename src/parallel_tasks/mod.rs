mod lua_userdata;

use crate::error::{log_error, ErrorSender};
use anyhow::{format_err, Result};
use futures::StreamExt;
use futures::stream::FuturesUnordered;
use tokio::{select, task};
use tokio::task::JoinHandle;
use tokio::sync::mpsc;
use tracing::{debug, trace};

pub type TaskHandle = JoinHandle<Result<()>>;

type TaskReceiver = mpsc::UnboundedReceiver<TaskHandle>;

#[derive(Clone)]
pub struct TaskSender {
    tx: mpsc::UnboundedSender<TaskHandle>
}

impl TaskSender {
    pub fn send(&self, task: TaskHandle) -> Result<()> {
        self.tx.send(task)?;

        Ok(())
    }
}

pub fn start(err_tx: ErrorSender) -> (TaskSender, JoinHandle<()>)
{
    let (tx, rx) = mpsc::unbounded_channel();

    let handle = task::spawn(async move {
        run(rx, err_tx).await
    });

    let tx = TaskSender { tx };

    (tx, handle)
}

#[allow(clippy::integer_arithmetic)] // tokio::select!
async fn run(mut rx: TaskReceiver, err_tx: ErrorSender) {
    let mut tasks = FuturesUnordered::<TaskHandle>::new();

    loop {
        select! {
            Some(task) = rx.recv() => {
                trace!("Adding a new task...");
                tasks.push(task)
            },
            Some(result) = tasks.next() => {
                let result = result
                    .map_err(|e| format_err!(e))
                    .and_then(|r| r);

                match result {
                    Ok(_) => continue,
                    Err(e) => {
                        rx.close();
                        log_error(e);

                        log_errors! {
                            err_tx.send(format_err!("A task failed to finish."));
                        }
                    }
                }
            },
            else => {
                debug!("Waiting for tasks to finish...");

                while let Some(result) = tasks.next().await {
                    let result = result
                        .map_err(|e| format_err!(e))
                        .and_then(|r| r);

                    log_errors!(result);
                }

                break
            }
        }
    }
}
