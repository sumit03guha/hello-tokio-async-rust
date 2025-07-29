use tokio::fs;

#[tokio::main]
async fn main() {
    let contents = fs::read_to_string("hello.txt").await.unwrap();
    println!("The contents of the file : {contents}");
}
