fn main() {
    // The closure will use Fn if it can because it prefers to only use a reference.
    // However, the spawn() method requires an FnOnce to be used,
    // which means to take by value. The move keyword lets us force the closure to take by
    // value instead of reference, and now that it owns the String, there are no lifetime problems anymore.
    let my_string = String::from("Can I go inside the thread?");
    let handle = std::thread::spawn(move || {
        println!("{my_string}");
    });
    handle.join().unwrap();

    let mut join_handles = vec![];
    for num in 0..10 {
        let handle = std::thread::spawn(move || {
            println!("Inside thread number: {num}");
        });
        join_handles.push(handle);
    }
    for handle in join_handles {
        handle.join().unwrap();
    }
}
