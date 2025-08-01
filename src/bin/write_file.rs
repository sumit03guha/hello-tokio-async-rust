use tokio::fs;

#[tokio::main]
async fn main() {
    let contents = String::from("World!");
    fs::write("world.txt", contents).await.unwrap();
}
