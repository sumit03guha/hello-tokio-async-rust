use std::{thread::sleep, time::Duration};

use tokio::time::timeout;

#[tokio::main]
async fn main() {
    let result = timeout(Duration::from_secs(3), hello()).await;

    match result {
        Ok(()) => println!("operation succeeded"),
        Err(e) => eprintln!("operation timed out: {e}"),
    }
}

async fn hello() {
    // sleep(Duration::from_secs(5)); // Blocks the main worker thread, so the timeout funtion gets disabled and fails to poll the `hello` future
    tokio::time::sleep(Duration::from_secs(5)).await;
    println!("Hello");
}
