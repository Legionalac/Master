use core::panic;
use std::thread;

fn main() {
    let new_thread = thread::spawn(|| {
        for _ in 1..5 {
            println!("Hello from the spawned thread");
        }
    });

    match new_thread.join() {
        Ok(_) => {}
        Err(error) => panic!("{:?}", error),
    }

    for _ in 1..3 {
        println!("Hello from the main thread");
    }
}
