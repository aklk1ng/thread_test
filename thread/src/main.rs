use std::thread;
use std::time::Duration;
fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} form the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    // the main thread will blocking until the handle thread done
    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
    handle.join().unwrap();
}
