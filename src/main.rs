use tokio::task;

#[tokio::main]
async fn main() {
    task::spawn(async {
        println!("Rustedserver (Temporary) - Version 0.1.0.");
    });
}
