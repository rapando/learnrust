use std::thread;
use std::time::Duration;

pub fn run_threads() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("hi, number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {i} from main thread");
        thread::sleep(Duration::from_millis(1));
    }
}

pub fn waiting_with_join() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("Hi number {i} from the spawned thread");
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {i} from main thread");
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}

pub fn with_closures() {
    let v = vec![1,2,3];
    // the ownership of v is moved to the closure
    let handle = thread::spawn(move || {
        println!("Here's a vector: {v:?}");
    });
    handle.join().unwrap();
}
