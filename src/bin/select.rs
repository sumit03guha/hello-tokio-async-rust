use std::time::Duration;

use tokio::{select, time::sleep};

#[tokio::main]
async fn main() {
    let task1 = tokio::spawn(async {
        sleep(Duration::from_secs(1)).await;
        "Task 1 executed"
    });
    let task2 = tokio::spawn(async {
        // sleep(Duration::from_secs(1)).await;
        "Task 2 executed"
    });

    select! {
        result = task1 => {
            println!("{:?}", result);
        }
        result = task2 => {
            println!("{:?}", result);
        }
    }
}
