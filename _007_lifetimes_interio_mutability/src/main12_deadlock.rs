use std::sync::Mutex;
fn main() {
    // deadlock:
    // let my_mutex = Mutex::new(5);
    // let mut mutex_changer = my_mutex.lock().unwrap();
    // let mut other_mutex_changer = my_mutex.lock().unwrap();
    // println!("This will never print...");

    // Handling deadlock:
    let my_mutex = Mutex::new(5);
    let mut mutex_changer = my_mutex.lock().unwrap();
    let mut other_mutex_changer = my_mutex.try_lock();
    if let Ok(value) = other_mutex_changer {
        println!("The MutexGuard has: {value}")
    } else {
        println!("Didn't get the lock")
    }

    let my_mutex = Mutex::new(5);
    // you never create a variable that holds the lock, so you don’t need to call drop().
    //You can do it 100 times if you want, and it doesn’t matter because no variable ever holds the lock:
    *my_mutex.lock().unwrap() = 6; // don’t need to make a variable to change the Mutex;  don’t need to call drop()

    let my_mutex = Mutex::new(5);
    for _ in 0..100 {
        *my_mutex.lock().unwrap() += 1;
    }
    println!("{}",my_mutex.try_lock().unwrap())
}
