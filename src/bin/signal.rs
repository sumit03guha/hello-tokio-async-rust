use tokio::signal;

#[tokio::main]
async fn main() {
    match signal::ctrl_c().await {
        Ok(()) => println!("Exiting"),
        Err(e) => println!("Errored : {}", e),
    }
}
