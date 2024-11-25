use std::sync::Arc;

use semaphore;

fn main() {
    let current_thread = 0;
    // USING SOMETHING DIFFERENT FOR RESOURCE??
    let order = Arc::new(semaphore::Semaphore::new(1, current_thread));

    let first_pointer = Arc::clone(&order);

    let first_thread = std::thread::spawn(move || {
        let mut i = 0;

        loop {
            if i > 100 {
                break;
            }
            match first_pointer.try_access() {
                Ok(_data) => {
                    println!("Hello from first thread : {}", i);
                    i = i + 1;
                    continue;
                }
                Err(_err) => {
                    continue;
                }
            }
        }
    });

    let second_pointer = Arc::clone(&order);

    let second_pointer = std::thread::spawn(move || {
        let mut i = 0;

        loop {
            if i > 100 {
                break;
            }
            match second_pointer.try_access() {
                Ok(_data) => {
                    println!("Hello from second thread : {}", i);
                    i = i + 1;
                    continue;
                }
                Err(_err) => {
                    continue;
                }
            }
        }
    });

    let third_pointer = Arc::clone(&order);

    let third_pointer = std::thread::spawn(move || {
        let mut i = 0;

        loop {
            if i > 100 {
                break;
            }
            match third_pointer.try_access() {
                Ok(_data) => {
                    println!("Hello from third thread : {}", i);
                    i = i + 1;
                    continue;
                }
                Err(_err) => {
                    continue;
                }
            }
        }
    });

    first_thread.join().unwrap();
    second_pointer.join().unwrap();
    third_pointer.join().unwrap();
}
