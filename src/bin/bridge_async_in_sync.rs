fn main() {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async_fn());
}

async fn async_fn() {
    println!("Hello World!");
}
