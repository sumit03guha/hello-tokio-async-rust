use std::sync::{Arc, Mutex as StdMutex};

use tokio::sync::Mutex as TokioMutex;

#[tokio::main]
async fn main() {
    let std_mutex_a = Arc::new(StdMutex::new(10));
    let tokio_mutex_a = Arc::new(TokioMutex::new(10));

    let std_mutex_a_clone = std_mutex_a.clone();
    let tokio_mutex_a_clone = tokio_mutex_a.clone();

    let std_mutex_handle = tokio::spawn(async move {
        *std_mutex_a_clone.lock().unwrap() += 1;
    });

    let tokio_mutex_handle = tokio::spawn(async move {
        *tokio_mutex_a_clone.lock().await += 1;
    });

    std_mutex_handle.await.unwrap();
    tokio_mutex_handle.await.unwrap();

    println!("a : {:#?}", std_mutex_a);
    println!("a : {:#?}", tokio_mutex_a);
}
