use std::{sync::Mutex, thread};
fn main() {
    thread::spawn(|| {}); // Do more thread stuff.
    thread::spawn(|| {}); // Do more thread stuff
                          // Don’t forget to join them here; otherwise, main() might end before the threads do.
                          // With scoped threads, you start with a scope, using thread::scope(). The threads will
                          // only live inside there. Then you use the closure that scope gives you to spawn the threads:

    thread::scope(|s| {
        s.spawn(|| {});
        s.spawn(|| {});
    }); // The threads automatically join here, so there’s no need to think about JoinHandles.

    // You still need a Mutex because more than one thread is
    // changing my_number, but you don’t need an Arc anymore. You don’t need to use move
    // because the threads aren’t forced to take ownership: they can just borrow the values
    // because the threads are guaranteed to not exist after the scope is over:
    let my_number = Mutex::new(0);
    thread::scope(|s: &thread::Scope| {
        s.spawn(|| {
            for _ in 0..10 {
                *my_number.lock().unwrap() += 1;
            }
        });
        s.spawn(|| {
            for _ in 0..10 {
                *my_number.lock().unwrap() += 1;
            }
        });
    });
}
