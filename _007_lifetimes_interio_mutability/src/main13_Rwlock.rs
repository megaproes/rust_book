use std::sync::RwLock;
fn main() {
    //deadlock:
    //     let my_rwlock = RwLock::new(5);
    //     let read1 = my_rwlock.read().unwrap();
    //     let read2 = my_rwlock.read().unwrap();
    //     println!("{read1:?}, {read2:?}");
    //     let write1 = my_rwlock.write().unwrap();

    // This type of lock allows a number of readers or at most one writer at any point in time.
    // The write portion of this lock typically allows modification of the underlying data
    // (exclusive access) and the read portion of this lock typically allows for read-only access (shared access).
    let lock = RwLock::new(5);

    // many reader locks can be held at once
    {
        let r1 = lock.read().unwrap();
        let r2 = lock.read().unwrap();
        assert_eq!(*r1, 5);
        assert_eq!(*r2, 5);
    } // read locks are dropped at this point

    // only one write lock may be held, however
    {
        let mut w = lock.write().unwrap();
        *w += 1;
        assert_eq!(*w, 6);
    } // write lock is dropped here

    let my_rwlock = RwLock::new(5);
    let read1 = my_rwlock.read().unwrap();
    let read2 = my_rwlock.read().unwrap();
    println!("{read1:?}, {read2:?}");
    println!("without variable read1 = {}", *my_rwlock.read().unwrap());
    drop(read1);
    drop(read2);
    // We dropped both readers, so we can use .write().
    let mut write1 = my_rwlock.write().unwrap();
    *write1 = 6;
    drop(write1);
    println!("{:?}", my_rwlock);
    
    // deadlock handling
    let my_rwlock = RwLock::new(5);
    let read1 = my_rwlock.read().unwrap();
    let read2 = my_rwlock.read().unwrap();
    if let Ok(mut number) = my_rwlock.try_write() {
        *number += 10;
        println!("Now the number is {}", number);
    } else {
        println!("Couldn't get write access, sorry!")
    };
}
