use std::time::Duration;

use hello_tokio_async_rust::utils::get_logger;
use tokio::time::{self, sleep};
use tokio_util::{sync::CancellationToken, task::TaskTracker};

#[tokio::main]
async fn main() {
    let sub = get_logger();
    tracing::subscriber::set_global_default(sub).unwrap();

    let tracker = TaskTracker::new();
    let cancellation_token = CancellationToken::new();

    for i in 0..10 {
        let cloned_cancellation_token = cancellation_token.clone();
        tracker.spawn(async move {
            tracing::info!("Spawning => {i}");
            tokio::select! {
                _ = background_task(i) => {
                    tracing::info!("Called background task with iteration : {i}");
                }
                _ = cloned_cancellation_token.cancelled() => {
                    tracing::info!("Cancellation token received");
                }
            }
        });
    }

    tokio::spawn(async move {
        tracing::info!("Spawed cancelling task. Going to sleep");
        time::sleep(Duration::from_secs(5)).await;
        cancellation_token.cancel();
    });

    tracker.close();
    tracker.wait().await;
}

async fn background_task(num: u32) {
    for i in 0..10 {
        let sleep_time = 100 * num as u64;
        tracing::info!("Background task : {num}; iteration : {i}. Going to sleep for {sleep_time}");
        sleep(Duration::from_millis(sleep_time)).await;
    }
}
