/*
 * Like in Go, we can use channels for communication between threads
 * a channel has two halves: a transmitter and receiver.
 */

// mpsc: multiple producer, single consumer. meaning our channel can have multiple producers but
// just one consumer.

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub fn calculations() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        // we're spawning a thread and moving tx's ownership here in order to send a value into it.
        let val = String::from("hi");
        tx.send(val).unwrap(); // after here, val's not usable anymore because its ownership was
                               // taken by tx.send
    });

    // we receive the value from the receiver channel here
    // there are two options, recv and try_recv
    // recv blocks the main thread's execution until a value is sent down the channel
    // try_recv does not block the main thread's execution
    let received = rx.recv().unwrap();
    println!("Got: {received}");
}

pub fn multiplevalues() {
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone(); // we're creating a copy of the transmitter channel
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
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
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {received}");
    }
}

/*
 * Using mutexes to alow access of data from one thread at a time
 * Mutex: mutual exclusion allows only one thread to access some data
 */

use std::sync::{Arc, Mutex};

pub fn mutex() {
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
