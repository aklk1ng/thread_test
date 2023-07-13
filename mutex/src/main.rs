use std::sync::{Arc, Mutex};
use std::thread;

#[allow(dead_code)]
fn test1() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
    println!("Result: {}", *counter.lock().unwrap());
}

fn test2() {
    let m = Mutex::new(5);
    let mut num = m.lock().unwrap();
    *num = 6;
    drop(num);
    let mut num1 = m.lock().unwrap();
    *num1 = 7;
    // drop(num1);

    println!("m = {:?}", m);
}

fn main() {
    test2();
}
