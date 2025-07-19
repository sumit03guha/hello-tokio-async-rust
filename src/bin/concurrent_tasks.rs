use std::time::Duration;

use tokio::time::sleep;

#[tokio::main]
async fn main() {
    println!("Hello, main!");

    // This is blocking

    // hello().await;
    // world().await;

    // Non blocking : Spawning separate threads for each
    let hello_handle = tokio::spawn(async {
        hello().await;
    });
    let world_handle = tokio::spawn(async {
        world().await;
    });

    hello_handle.await.unwrap();
    world_handle.await.unwrap();
}

async fn hello() {
    println!("Hello going to sleep");
    sleep(Duration::from_secs(5)).await;
    println!("Hello awake");
}

async fn world() {
    println!("World");
}
