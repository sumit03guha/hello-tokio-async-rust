use hello_tokio_async_rust::utils::get_logger;
use tokio::sync::OnceCell;

static VALUE: OnceCell<u32> = OnceCell::const_new();

// This gets called only once while setting the value for the first time
async fn init_value() -> u32 {
    tracing::info!("Setting value");
    101
}

async fn get_value() -> &'static u32 {
    VALUE.get_or_init(init_value).await
}

#[tokio::main]
async fn main() {
    let sub = get_logger();
    tracing::subscriber::set_global_default(sub).unwrap();

    let a = get_value().await;
    let b = get_value().await;
    let c = get_value().await;

    tracing::info!("a : {}", a);
    tracing::info!("b : {}", b);
    tracing::info!("c : {}", c);
}
