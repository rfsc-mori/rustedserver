#![forbid(unsafe_code)]

mod settings;
mod settings_loader;
mod server_state;

use anyhow::Result;
use tokio::task;
use tracing::Level;
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
}

#[tokio::main]
async fn main() -> Result<()> {
    setup_tracing().expect("Failed to setup tracing.");

    task::spawn_blocking(print_server_information).await?;

    let settings = task::spawn(async {
        settings_loader::load_settings().await
            .expect("Failed to load configuration.")
    });

    let server_state = task::spawn(async move {
        match settings.await {
            Ok(settings) => {
                server_state::init_state(settings).await
                    .expect("Failed to initialize server state.")
            },
            Err(e) => panic!(e)
        }
    });

    server_state.await?;

    Ok(())
}
