// Unsafe:
// - database::compat::query_result::QueryResult::new:
// * Reason: OwningHandle::new_with_fn, pack a SQLx query and its result into an UserData with Arc<>.
//
#![deny(unsafe_code)]

// Features:
#![feature(iterator_fold_self)]
#![feature(trait_alias)]

// Clippy:
#![warn(
clippy::all,
clippy::cargo,
clippy::complexity,
clippy::correctness,
clippy::nursery,
clippy::pedantic,
clippy::perf,
clippy::restriction,
clippy::style,
)]

// Other allows are in specific modules/functions.
#![allow(
clippy::cargo_common_metadata,
clippy::implicit_return,
clippy::missing_const_for_fn,
clippy::missing_docs_in_private_items,
clippy::module_name_repetitions,
clippy::multiple_crate_versions,
clippy::single_match_else,
clippy::wildcard_enum_match_arm,
clippy::integer_arithmetic
)]

#[macro_use]
mod error;

mod database;
mod parallel_tasks;
mod scripting;
mod server;
mod settings;

use crate::error::log_error;
use crate::settings::Settings;
use anyhow::{Context, Result};
use std::path::PathBuf;
use tokio::task;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

fn setup_tracing() -> Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();

    tracing::subscriber::set_global_default(subscriber)?;

    Ok(())
}

#[allow(clippy::print_stdout)] // Print server information.
fn display_server_information() {
    println!("Rustedserver (Temporary) - Version 0.1.0.");
    println!();
}

#[tokio::main]
async fn main() -> Result<()> {
    setup_tracing()
        .context("Failed to setup tracing.")?;

    log_errors! {
        run().await;
    };

    Ok(())
}

async fn run() -> Result<()> {
    task::spawn_blocking(display_server_information).await?;

    info!("Loading settings...");

    let settings = Settings::load_from_dir(PathBuf::from("./config/"))
        .await
        .context("Failed to load settings.")?;

    info!("Starting the server...");
    server::run(settings).await?;

    info!("Server stopped.");

    Ok(())
}
