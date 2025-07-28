use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let (sender, mut receiver) = mpsc::channel(5);

    for i in 1..=10 {
        let sender_clone = sender.clone();

        tokio::spawn(async move {
            if let Err(e) = sender_clone.send(i).await {
                println!("Error sending at {i} : {e}");
            }
        });
    }

    drop(sender);

    while let Some(received_value) = receiver.recv().await {
        println!("Received : {received_value}");
    }
}
