use std::sync::{Arc, Mutex};
use std::thread;

struct MyBox(*const u8);
unsafe impl Sync for MyBox {}

fn main() {
    let b = &MyBox(5 as *const u8);
    let v = Arc::new(Mutex::new(b));
    let t = thread::spawn(move || {
        let _v1 = v.lock().unwrap();
    });
    t.join().unwrap();
}
