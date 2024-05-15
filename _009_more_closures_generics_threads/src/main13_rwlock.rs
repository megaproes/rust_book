// When to Use RwLock

// Use RwLock when:
// -- You have a read-heavy workload where multiple threads frequently read shared data but do not often modify it.
// -- You want to allow multiple threads to read shared data concurrently to improve performance.

use std::sync::{Arc, RwLock};
use std::thread;

fn main() {
    let data = Arc::new(RwLock::new(0));
    let mut handles = vec![];

    // Spawn threads for reading
    for _ in 0..10 {
        let data_clone = Arc::clone(&data);
        let handle = thread::spawn(move || {
            let read_guard = data_clone.read().unwrap();
            println!("Read: {}", *read_guard);
            // Read guard is dropped here
        });
        handles.push(handle);
    }

    // Spawn threads for writing
    for _ in 0..2 {
        let data_clone = Arc::clone(&data);
        let handle = thread::spawn(move || {
            let mut write_guard = data_clone.write().unwrap();
            *write_guard += 1;
            println!("Wrote: {}", *write_guard);
            // Write guard is dropped here
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
