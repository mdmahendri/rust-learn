use std::thread;
use std::time::Duration;
use std::sync::{Arc, Mutex};

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..5 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..3 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
    println!("Hello, world!");

    let v = vec![1, 2, 3];
    let handle2  = thread::spawn(move || {
        println!("Here's a vector: {:?}", v)
    });
    handle2.join().unwrap();

    // add Arc to share ownership of the counter across threads
    // mutex is used to ensure that only one thread can access the counter at a time
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
