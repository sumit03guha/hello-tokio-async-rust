use std::time::Duration;

use tokio::time::interval;

#[tokio::main]
async fn main() {
    let mut ticker = interval(Duration::from_secs(2));

    loop {
        ticker.tick().await; // this ticks every 2 secs
        hello().await;
    }
}

async fn hello() {
    println!("Hello World!");
}
