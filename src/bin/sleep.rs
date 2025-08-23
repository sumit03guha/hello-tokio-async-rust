use std::{thread::sleep, time::Duration};

// #[tokio::main(flavor = "current_thread")] // with a single thread the `hello` function blocks the thread for 3 secs.
#[tokio::main]
async fn main() {
    // Tick every 250 ms in the background
    let join_handle = tokio::spawn(async {
        loop {
            // sleep(Duration::from_millis(250)); // os thread; blocks the spwaned thread and the loops goes on forever
            tokio::time::sleep(Duration::from_millis(250)).await; // async future that is dropped as soon as the root future finishes if not awaited
            println!("tick");
        }
    });

    hello().await;
    // join_handle.await.unwrap();
    println!("world");
}

async fn hello() {
    println!("Inside hello");
    // tokio::time::sleep(Duration::from_secs(3)).await; // ← async version
    sleep(Duration::from_secs(3));
    println!("Hello");
}
