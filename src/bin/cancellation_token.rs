use std::time::Duration;

use tokio::time::sleep;
use tokio_util::sync::CancellationToken;

#[tokio::main]
async fn main() {
    let cancellation_token = CancellationToken::new();
    let cloned_cancellation_token = cancellation_token.clone();

    let handle = tokio::spawn(async move {
        tokio::select! {
            _ = cloned_cancellation_token.cancelled() => {
                println!("received cancellation signal");
            }
            _ = sleep(Duration::from_secs(5)) => {
                println!("going to sleep");
            }
        }
    });

    tokio::spawn(async move {
        println!("Cancelling...");
        cancellation_token.cancel();
    });

    handle.await.unwrap();
}
