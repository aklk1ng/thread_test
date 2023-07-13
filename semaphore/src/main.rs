use std::sync::Arc;

use tokio::sync::Semaphore;

#[tokio::main]
async fn main() {
    let semaphore = Arc::new(Semaphore::new(3));
    let mut join_handle = Vec::new();

    for i in 0..5 {
        let permit = semaphore.clone().acquire_owned().await.unwrap();
        join_handle.push(tokio::spawn(async move {
            println!("{}", i);
            drop(permit);
        }));
    }

    for handle in join_handle {
        handle.await.unwrap();
    }
}
