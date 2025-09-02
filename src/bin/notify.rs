use std::sync::Arc;

use hello_tokio_async_rust::utils::get_logger;
use tokio::sync::Notify;

#[tokio::main]
async fn main() {
    let sub = get_logger();
    tracing::subscriber::set_global_default(sub).unwrap();

    let notification_receiver = Arc::new(Notify::new());
    let notifier = notification_receiver.clone();

    let notification_receiver_handle = tokio::spawn(async move {
        tracing::info!("Started to listen to notification");
        notification_receiver.notified().await;
        tracing::info!("Notification received");
    });

    let notification_sender_handle = tokio::spawn(async move {
        tracing::info!("Sending notification");
        notifier.notify_one();
        tracing::info!("Notification sent");
    });

    let _ = tokio::join!(notification_sender_handle, notification_receiver_handle);
}
