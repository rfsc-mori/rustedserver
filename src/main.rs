mod settings;
mod settings_loader;
mod server_state;

use anyhow::Result;
use tokio::task;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

fn print_info() {
    println!("Rustedserver (Temporary) - Version 0.1.0.");
}

#[tokio::main]
async fn main() -> Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::DEBUG)
        .finish();

    tracing::subscriber::set_global_default(subscriber)?;

    task::spawn_blocking(print_info).await?;

    let settings = task::spawn(async {
        settings_loader::load_config().await
            .expect("Failed to load server configuration.")
    });

    let server = task::spawn(async move {
        match settings.await {
            Ok(settings) => {
                server_state::init_state(settings).await
                    .expect("Failed to initialize server.")
            },
            Err(e) => panic!(e)
        }
    });

    server.await.unwrap();

    Ok(())
}
