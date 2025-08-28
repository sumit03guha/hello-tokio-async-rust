use std::{task::Poll, thread, time::Duration};

use tokio::time::Instant;

struct Delay {
    when: Instant,
}

#[tokio::main]
async fn main() {
    println!("Let's go!");
    let when = Instant::now() + Duration::from_secs(3);
    let delay = Delay { when };

    delay.await;

    println!("Done")
}

impl Future for Delay {
    type Output = &'static str;
    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        if self.when <= Instant::now() {
            Poll::Ready("Ready")
        } else {
            let when = self.when;
            let waker = cx.waker().clone();
            thread::spawn(move || {
                if when > Instant::now() {
                    thread::sleep(when - Instant::now());
                }
                waker.wake();
            });
            Poll::Pending
        }
    }
}
