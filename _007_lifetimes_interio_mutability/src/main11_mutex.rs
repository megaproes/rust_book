use std::sync::Mutex;
fn main() {
    let my_mutex: Mutex<i32> = Mutex::new(5);
    let mut mutex_changer: std::sync::MutexGuard<i32> = my_mutex.lock().unwrap();
    println!("{my_mutex:?}");
    println!("{mutex_changer:?}");
    *mutex_changer = 6;
    println!("{mutex_changer:?}");

    let my_mutex = Mutex::new(5);
    {
        let mut mutex_changer = my_mutex.lock().unwrap();
        *mutex_changer = 6;
    }
    println!("{my_mutex:?}");

    let my_mutex = Mutex::new(5);
    let mut mutex_changer = my_mutex.lock().unwrap();
    *mutex_changer = 6;
    drop(mutex_changer); // This drops mutex_changer. It is now gone, and my_mutex is unlocked.
    println!("{my_mutex:?}");
}
