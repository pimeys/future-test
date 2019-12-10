use tokio::runtime::Runtime;
use std::future::Future;
use futures::future::{self, FutureExt};

/// This is very similar what test_two does, but with less sugar. (you can't await here)
fn test_one() -> impl Future<Output = ()> {
    println!("called test_one()");
    future::ready(())
}

/// Here we allow awaiting, but for simplicity we don't await anything.
async fn test_two() {
    println!("called test_two()");
}

/// This is using the old style combinators to see how we create only one
/// implementation of a Future
fn test_three() -> impl Future<Output = ()> {
    println!("called test_three()");
    test_one().then(|_| test_two())
}

/// Same as test_three, but with async/await
async fn test_four() {
    println!("called test_four()");
    test_one().await;
    test_two().await;
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut rt = Runtime::new()?; // <- this is a thread-pool (one IO thread per core)
    let some_result = test_two(); // Do we allocate here? No.

    rt.block_on(test_one()); // <- malloc() for future closure
    rt.block_on(test_two()); // <- malloc() for future closure
    rt.block_on(test_three()); // <- malloc() for future closure
    rt.block_on(test_four()); // <- malloc() for future closure
    rt.block_on(some_result); // <- ???

    Ok(())
}
