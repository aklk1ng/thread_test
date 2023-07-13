use std::sync::mpsc;
use std::thread;
use std::time::Duration;

#[allow(dead_code)]
fn test1() {
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_millis(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}

fn test2() {
    let (send, recv) = mpsc::channel();
    let num_threads = 3;
    for i in 0..num_threads {
        let thread_send = send.clone();
        thread::spawn(move || {
            thread_send.send(i).unwrap();
            println!("thread {:?} finished", i);
        });
    }

    // if didn't drop the sender
    // the channel will not be closed
    // so the main thread will be block
    drop(send);
    for x in recv {
        println!("Got: {}", x);
    }
    println!("finished iterating");
}

fn main() {
    test2();
}
