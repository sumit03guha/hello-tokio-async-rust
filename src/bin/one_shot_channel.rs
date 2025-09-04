use tokio::sync::oneshot;

#[tokio::main]
async fn main() {
    let (sender, receiver) = oneshot::channel();

    tokio::spawn(async move {
        sender.send("Hello").unwrap();
    });

    match receiver.await {
        Ok(message) => println!("Received message : {}", message),
        Err(e) => eprintln!("Error receiving message : {}", e),
    }
}
