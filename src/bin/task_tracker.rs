use hello_tokio_async_rust::utils::get_logger;
use tokio_util::task::TaskTracker;

#[tokio::main]
async fn main() {
    let sub = get_logger();
    tracing::subscriber::set_global_default(sub).unwrap();

    let tracker = TaskTracker::new();

    for i in 0..10 {
        tracker.spawn(async move {
            tracing::info!("Spawned : {}", i);
        });
    }

    tracker.close();
    tracker.wait().await;
}
