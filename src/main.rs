use std::fs;
use std::path::Path;
use tokio::task;

#[tokio::main]
async fn main() {
    task::spawn(async {
        println!("Rustedserver (Temporary) - Version 0.1.0.");

        if !Path::new("./config.lua").exists() {
            fs::copy("./config.lua.dist", "./config.lua")
                .expect("Failed to create config.lua");
        }
    });
}
