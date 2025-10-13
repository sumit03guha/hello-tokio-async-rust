#[tokio::main]
async fn main() {}

async fn add_one(num: u32) -> u32 {
    num + 1
}

#[tokio::test]
async fn test_add_one() {
    let num = 2;
    let result = add_one(num).await;

    assert_eq!(num, result - 1);
}
