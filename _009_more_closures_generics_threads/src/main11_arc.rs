use std::sync::{Arc, Mutex};

fn main() {
    let my_number = Arc::new(Mutex::new(0)); // Shared, mutable state with thread safety
    let mut handle_vec = vec![];

    for _ in 0..10 {
        let my_number_clone = Arc::clone(&my_number); // Clone the Arc for thread-safe shared ownership
        let handle = std::thread::spawn(/*move*/|| {
            for _ in 0..10 {
                *my_number_clone.lock().unwrap() += 1; // Lock the Mutex to safely access and mutate the data
            }
        });
        
        handle_vec.push(handle);
    } // my_number_clone is dropped here, so without move, the closure may outlive the current scope (for-loop) and have dangling reference to my_number_clone
    
    handle_vec
        .into_iter()
        .for_each(|handle| handle.join().unwrap()); // Wait for all threads to finish

    println!("{:?}", my_number); // Print the final value
}
