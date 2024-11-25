use std::{
    sync::{Arc, Mutex},
    thread,
};

fn main() {
    let data = vec!['A', 'b', 'c', 'D'];
    let pointer = Arc::new(Mutex::new(data));

    let first_pointer = Arc::clone(&pointer);
    let first_thread = thread::spawn(move || {
        let data = first_pointer.lock().unwrap();
        println!("Unchanged vec : {:?}", *data)
    });

    let second_pointer = Arc::clone(&pointer);
    let second_thread = thread::spawn(move || {
        let mut data = second_pointer.lock().unwrap();
        *data = data
            .iter()
            .map(|letter: &char| {
                if letter.is_ascii_uppercase() {
                    return letter.to_ascii_lowercase();
                } else {
                    return letter.to_ascii_uppercase();
                }
            })
            .collect();
        println!("Changed vec : {:?}", *data)
    });

    first_thread.join().unwrap();
    second_thread.join().unwrap();
}
