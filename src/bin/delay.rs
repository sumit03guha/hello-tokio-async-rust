use std::{task::Poll, thread, time::Duration};
use tokio::time::Instant;

use hello_tokio_async_rust::utils::get_logger;

struct Delay {
    when: Instant,
    spawned: bool,
}

#[tokio::main]
async fn main() {
    let sub = get_logger();
    tracing::subscriber::set_global_default(sub).unwrap();

    tracing::info!("Let's go!");

    let when = Instant::now() + Duration::from_secs(3);
    let delay = Delay {
        when,
        spawned: false,
    };

    let res = delay.await;
    tracing::info!("res : {}", res);

    tracing::info!("Done")
}

impl Future for Delay {
    type Output = &'static str;
    fn poll(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        if self.when <= Instant::now() {
            tracing::info!("Ready");
            Poll::Ready("Ready")
        } else {
            if !self.spawned {
                self.spawned = true;
                let when = self.when;
                let waker = cx.waker().clone();
                thread::spawn(move || {
                    if when > Instant::now() {
                        tracing::info!("Going to sleep");
                        thread::sleep(when - Instant::now());
                    }
                    waker.wake();
                });
            }
            tracing::info!("Pending");
            Poll::Pending
        }
    }
}
