use tokio_stream::StreamExt;

#[tokio::main]
async fn main() {
    let a = [1, 2, 3, 4, 5];

    let mut stream = tokio_stream::iter(&a);

    while let Some(i) = stream.next().await {
        println!("Got : {i}");
    }
}
