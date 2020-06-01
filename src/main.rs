mod settings;
mod settings_loader;
mod server_state;

use crate::settings::Settings;

use anyhow::Result;
use tokio::task;

fn print_info() {
    println!("Rustedserver (Temporary) - Version 0.1.0.");
}

async fn server_init(_: Settings) -> Result<()> {
    Ok(())
}

#[tokio::main]
async fn main() {
    task::spawn_blocking(print_info).await
        .expect("Failed to show server info.");

    let settings = task::spawn(async {
        settings_loader::load_config().await
            .expect("Failed to load server configuration.")
    });

    let server = task::spawn(async move {
        server_init(settings.await?).await
    });

    server.await.expect("Failed to initialize server.");
}
