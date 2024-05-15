use std::sync::Mutex;
use std::thread;
fn main() {
    let mutex_number = Mutex::new(0);
    let mut regular_mut_number = 0;
    let regular_unmut_number = 0;
    thread::scope(|s| {
        s.spawn(|| {
            for _ in 0..3 {
                *mutex_number.lock().unwrap() += 1;
                regular_mut_number += 1;
                println!("Multiple immutable borrows is fine! {regular_unmut_number}");
            }
        });
        s.spawn(|| {
            for _ in 0..3 {
                *mutex_number.lock().unwrap() += 1;
                // regular_mut_number += 1;
                println!("Borrowing {regular_unmut_number} here too, it's just fine!");
            }
        });
    });
    println!("mutex_number: {mutex_number:?}");
    println!("regular_mut_number: {regular_mut_number}");
    println!("regular_unmut_number: {regular_unmut_number}");

    //  So, if you are okay with your threads only living inside a single scope, be sure to check
    // out scoped threads. Regular threads have the advantage of living forever as long as
    // your program is running, but scoped threads are easy to spawn and use if you have a
    // task to accomplish and don’t need them after the task is done.
    thread::scope(|s| {
        for thread_number in 0..1000 {
            s.spawn(move || {
                println!("Thread number {thread_number}");
            });
        }
    });
}

// 					Regular Threads

// Regular threads in Rust are created using std::thread::spawn. They run independently of the main thread and have their own lifetime.
// To ensure safety, Rust enforces that any data shared between threads must be thread-safe (i.e., must implement the Send and Sync traits).
// Additionally, the data must be 'static, meaning it must live for the entire duration of the program, or be explicitly managed
// using smart pointers like Arc or Mutex.

// 					**Scoped Threads**

// Scoped threads, created using std::thread::scope, are a newer addition to Rust’s concurrency toolbox.
// Scoped threads allow you to spawn threads that are guaranteed to join before the scope exits,
// ensuring they complete their work and preventing dangling references. Scoped threads do not require 'static lifetimes,
// which simplifies the sharing of non-'static data between threads.

// 					Key Benefits of **Scoped Threads**

//  **Automatic Joining**:
//  -- Scoped threads automatically join (i.e., wait for all spawned threads to complete) at the end of the scope,
// ensuring that all threads finish their execution before the program proceeds. This removes the need to manually join each thread,
// reducing boilerplate and potential errors.

//  **Non-'static Lifetimes**:

//    	 -- Unlike regular threads, scoped threads can borrow non-`'static data from the parent thread.
// This makes it easier to work with data that does not need to live for the entire duration of the program.

//     **Safety and Simplicity**:
//         -- Scoped threads provide a safer and simpler way to handle concurrency for tasks that are local to a specific scope.
// Since the threads are guaranteed to finish within the scope, there is no risk of data races or accessing invalid data after the scope ends.
