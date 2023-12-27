use std::{sync::Arc, thread};

fn main() {
    let data = Arc::new("hello there world");

    let mut handles = vec![];

    for i in 0..3 {
        let data_clone = Arc::clone(&data);
        let handle = thread::spawn(move || {
            println!("Data in thread: {} - {}", data_clone, i);
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
}
