use std::cell::RefCell;
use std::sync::{Arc, Barrier, Condvar, Mutex, Once};
use std::thread;
use std::time::Duration;

#[allow(dead_code)]
struct Foo;
impl Foo {
    thread_local! {
        static FOO: RefCell<usize> = RefCell::new(0);
    }
}

#[allow(dead_code)]
fn test1() {
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

#[allow(dead_code)]
fn test2() {
    let mut handles = Vec::with_capacity(6);
    let barrier = Arc::new(Barrier::new(6));

    for _ in 0..6 {
        let b = barrier.clone();
        handles.push(thread::spawn(move || {
            println!("before wait");
            b.wait();
            println!("after wait");
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

#[allow(dead_code)]
fn test3() {
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair2 = pair.clone();

    thread::spawn(move || {
        let (lock, cvar) = &*pair2;
        let mut started = lock.lock().unwrap();
        println!("changing started");
        *started = true;
        cvar.notify_one();
    });

    let (lock, cvar) = &*pair;
    let mut started = lock.lock().unwrap();
    while !*started {
        started = cvar.wait(started).unwrap();
    }
    println!("started changed");
}

static mut VAL: usize = 0;
// only be called once
static INIT: Once = Once::new();

#[allow(dead_code)]
fn test4() {
    let handle1 = thread::spawn(move || {
        INIT.call_once(|| unsafe {
            VAL = 1;
        })
    });

    let handle2 = thread::spawn(move || {
        INIT.call_once(|| unsafe {
            VAL = 2;
        })
    });
    handle1.join().unwrap();
    handle2.join().unwrap();
    println!("{}", unsafe { VAL });
}

fn main() {
    test3();
}
