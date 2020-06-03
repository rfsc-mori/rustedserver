#![forbid(unsafe_code)]

mod database;
mod settings;
mod settings_loader;
mod server_state;

use anyhow::{Context, Result};
use tracing::{Level, error};
use tracing_subscriber::FmtSubscriber;

fn setup_tracing() -> Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::DEBUG)
        .finish();

    tracing::subscriber::set_global_default(subscriber)?;

    Ok(())
}

fn print_server_information() {
    println!("Rustedserver (Temporary) - Version 0.1.0.");
    println!();
}

async fn run() -> Result<()> {
    setup_tracing()
        .expect("Failed to setup logging.");

    print_server_information();

    let settings = settings_loader::load_settings()
        .await
        .context("Failed to load server settings.")?;

    let server = server_state::init_state(settings)
        .await
        .context("Failed to initialize server state.")?;

    Ok(())
}

#[tokio::main]
async fn main() {
    if let Err(e) = run().await {
        error!("Error: {}", e);

        for source in e.chain().skip(1) {
            error!("Caused by: {}", source);
        }
    }
}
